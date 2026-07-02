//! Stable benchmark suite for representative `vstr` helpers.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::hint::black_box;
use std::process;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

const ITERATIONS: usize = 1_024;
const REPORT_SCHEMA: &str = "https://knifer-rs.dev/schemas/vstr-bench-report.v1";
const REPORT_VERSION: &str = "v1";
const BENCHMARK_SUITE: &str = "vstr_bench";
const UNKNOWN_ENV: &str = "unknown";

#[derive(Clone, Copy)]
struct BenchResult {
    name: &'static str,
    iterations: usize,
    elapsed_ns: u128,
    checksum: usize,
}

struct BenchEnvironment {
    rustc_version: String,
    target_triple: String,
    feature_set: String,
    commit_sha: String,
}

struct BenchInputs {
    baseline_source: String,
    max_regression_percent: String,
    run_mode: String,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ReportFormat {
    Plain,
    Json,
    Markdown,
}

enum Command {
    Report(ReportFormat),
    Compare {
        baseline_path: String,
        max_regression_bps: u128,
        format: CompareFormat,
    },
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CompareFormat {
    Json,
    Markdown,
}

#[derive(Clone, Copy)]
struct Comparison {
    name: &'static str,
    baseline_elapsed_ns: u128,
    current_elapsed_ns: u128,
    direction: ChangeDirection,
    change_bps: u128,
    failed: bool,
}

#[derive(Clone, Copy)]
enum ChangeDirection {
    Improvement,
    Regression,
    Unchanged,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let sample = "Knifer-RS hello rust world 你好 ".repeat(512);
    let path = "/api/v1/projects/knifer-rs/users/42";
    let command = command()?;

    let results = [
        run_case("contains", &sample, bench_contains),
        run_case("find_all", &sample, bench_find_all),
        run_case("replace_many", &sample, bench_replace_many),
        run_case("to_snake_case", &sample, bench_to_snake_case),
        run_case("wrap", &sample, bench_wrap),
        run_case("levenshtein_distance", &sample, bench_levenshtein),
        run_case("ant_path_match", path, bench_ant_path),
    ];

    match command {
        Command::Report(ReportFormat::Plain) => print_plain(&results),
        Command::Report(ReportFormat::Json) => print_json(&results),
        Command::Report(ReportFormat::Markdown) => print_markdown(&results),
        Command::Compare {
            baseline_path,
            max_regression_bps,
            format,
        } => {
            let comparisons = compare_results(&results, &baseline_path, max_regression_bps)?;
            match format {
                CompareFormat::Json => {
                    print_compare_json(&baseline_path, max_regression_bps, &comparisons);
                }
                CompareFormat::Markdown => {
                    print_compare_markdown(&baseline_path, max_regression_bps, &comparisons);
                }
            }
        }
    }

    Ok(())
}

fn command() -> Result<Command, String> {
    let args: Vec<String> = env::args().skip(1).filter(|arg| arg != "--bench").collect();
    match args.first().map(String::as_str) {
        Some("--json") => Ok(Command::Report(ReportFormat::Json)),
        Some("--markdown") => Ok(Command::Report(ReportFormat::Markdown)),
        Some("--compare-json") => compare_command(&args, CompareFormat::Json),
        Some("--compare-markdown") => compare_command(&args, CompareFormat::Markdown),
        Some(unknown) => Err(format!("unknown benchmark argument: {unknown}")),
        None => Ok(Command::Report(ReportFormat::Plain)),
    }
}

fn compare_command(args: &[String], format: CompareFormat) -> Result<Command, String> {
    let baseline_path = args
        .get(1)
        .ok_or_else(|| "missing baseline JSON path".to_owned())?
        .to_owned();
    let max_regression_pct = option_value(args, "--max-regression-pct").unwrap_or("20.00");
    let max_regression_bps = parse_percent_to_bps(max_regression_pct)?;

    Ok(Command::Compare {
        baseline_path,
        max_regression_bps,
        format,
    })
}

fn option_value<'arg>(args: &'arg [String], name: &str) -> Option<&'arg str> {
    args.windows(2)
        .find(|window| window[0] == name)
        .map(|window| window[1].as_str())
}

fn run_case(name: &'static str, input: &str, case: fn(&str) -> usize) -> BenchResult {
    let start = Instant::now();
    let mut checksum = 0usize;
    for _ in 0..ITERATIONS {
        checksum = checksum.wrapping_add(case(black_box(input)));
    }

    result(name, start.elapsed(), checksum)
}

fn result(name: &'static str, elapsed: Duration, checksum: usize) -> BenchResult {
    BenchResult {
        name,
        iterations: ITERATIONS,
        elapsed_ns: elapsed.as_nanos(),
        checksum,
    }
}

fn print_plain(results: &[BenchResult]) {
    let environment = bench_environment();
    let inputs = bench_inputs();
    println!(
        "suite={} schema={} version={} rustc={} target={} features={} commit={} baseline_source={} max_regression_percent={} run_mode={}",
        BENCHMARK_SUITE,
        REPORT_SCHEMA,
        REPORT_VERSION,
        environment.rustc_version,
        environment.target_triple,
        environment.feature_set,
        environment.commit_sha,
        inputs.baseline_source,
        inputs.max_regression_percent,
        inputs.run_mode
    );
    for result in results {
        println!(
            "bench={} iterations={} elapsed_ns={} checksum={}",
            result.name, result.iterations, result.elapsed_ns, result.checksum
        );
    }
}

fn print_json(results: &[BenchResult]) {
    let environment = bench_environment();
    let inputs = bench_inputs();
    let mut output = String::new();
    write!(
        output,
        "{{\"schema\":\"{REPORT_SCHEMA}\",\"suite\":\"{BENCHMARK_SUITE}\",\"version\":\"{REPORT_VERSION}\",\"environment\":{{\"rustc_version\":\"{}\",\"target_triple\":\"{}\",\"feature_set\":\"{}\",\"commit_sha\":\"{}\"}},\"inputs\":{{\"baseline_source\":\"{}\",\"max_regression_percent\":\"{}\",\"run_mode\":\"{}\"}},\"results\":[",
        json_escape(&environment.rustc_version),
        json_escape(&environment.target_triple),
        json_escape(&environment.feature_set),
        json_escape(&environment.commit_sha),
        json_escape(&inputs.baseline_source),
        json_escape(&inputs.max_regression_percent),
        json_escape(&inputs.run_mode),
    )
    .expect("write to String cannot fail");

    for (index, result) in results.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        write!(
            output,
            "{{\"bench\":\"{}\",\"iterations\":{},\"elapsed_ns\":{},\"checksum\":{}}}",
            result.name, result.iterations, result.elapsed_ns, result.checksum
        )
        .expect("write to String cannot fail");
    }

    output.push_str("]}");
    println!("{output}");
}

fn compare_results(
    current_results: &[BenchResult],
    baseline_path: &str,
    max_regression_bps: u128,
) -> Result<Vec<Comparison>, String> {
    let baseline = fs::read_to_string(baseline_path)
        .map_err(|error| format!("failed to read baseline JSON {baseline_path}: {error}"))?;
    let baseline_report: Value = serde_json::from_str(&baseline)
        .map_err(|error| format!("failed to parse baseline JSON {baseline_path}: {error}"))?;

    current_results
        .iter()
        .map(|result| {
            let baseline_elapsed_ns = baseline_elapsed_ns(&baseline_report, result.name)?;
            let (direction, change_bps) =
                change_from_baseline(result.elapsed_ns, baseline_elapsed_ns)?;
            let failed =
                matches!(direction, ChangeDirection::Regression) && change_bps > max_regression_bps;

            Ok(Comparison {
                name: result.name,
                baseline_elapsed_ns,
                current_elapsed_ns: result.elapsed_ns,
                direction,
                change_bps,
                failed,
            })
        })
        .collect()
}

fn baseline_elapsed_ns(report: &Value, bench_name: &str) -> Result<u128, String> {
    let results = report
        .get("results")
        .and_then(Value::as_array)
        .ok_or_else(|| "baseline JSON is missing results array".to_owned())?;

    let result = results
        .iter()
        .find(|entry| entry.get("bench").and_then(Value::as_str) == Some(bench_name))
        .ok_or_else(|| format!("baseline JSON is missing benchmark: {bench_name}"))?;

    result
        .get("elapsed_ns")
        .and_then(Value::as_u64)
        .map(u128::from)
        .ok_or_else(|| format!("baseline JSON has invalid elapsed_ns for: {bench_name}"))
}

fn change_from_baseline(
    current_elapsed_ns: u128,
    baseline_elapsed_ns: u128,
) -> Result<(ChangeDirection, u128), String> {
    if baseline_elapsed_ns == 0 {
        return Err("baseline elapsed_ns must be greater than zero".to_owned());
    }

    if current_elapsed_ns == baseline_elapsed_ns {
        return Ok((ChangeDirection::Unchanged, 0));
    }

    let (direction, delta) = if current_elapsed_ns > baseline_elapsed_ns {
        (
            ChangeDirection::Regression,
            current_elapsed_ns - baseline_elapsed_ns,
        )
    } else {
        (
            ChangeDirection::Improvement,
            baseline_elapsed_ns - current_elapsed_ns,
        )
    };

    Ok((
        direction,
        delta.saturating_mul(10_000) / baseline_elapsed_ns,
    ))
}

fn print_compare_json(baseline_path: &str, max_regression_bps: u128, comparisons: &[Comparison]) {
    let status = comparison_status(comparisons);
    let environment = bench_environment();
    let inputs = bench_inputs();
    let results: Vec<Value> = comparisons
        .iter()
        .map(|comparison| {
            json!({
                "bench": comparison.name,
                "baseline_elapsed_ns": json_u64(comparison.baseline_elapsed_ns),
                "current_elapsed_ns": json_u64(comparison.current_elapsed_ns),
                "direction": comparison.direction.as_str(),
                "change_percent": format_bps(comparison.change_bps),
                "status": if comparison.failed { "fail" } else { "pass" },
            })
        })
        .collect();

    println!(
        "{}",
        json!({
            "schema": REPORT_SCHEMA,
            "suite": BENCHMARK_SUITE,
            "version": REPORT_VERSION,
            "environment": {
                "rustc_version": environment.rustc_version,
                "target_triple": environment.target_triple,
                "feature_set": environment.feature_set,
                "commit_sha": environment.commit_sha,
            },
            "inputs": {
                "baseline_source": inputs.baseline_source,
                "max_regression_percent": inputs.max_regression_percent,
                "run_mode": inputs.run_mode,
            },
            "baseline": baseline_path,
            "max_regression_percent": format_bps(max_regression_bps),
            "status": status,
            "results": results,
        })
    );
}

fn print_compare_markdown(
    baseline_path: &str,
    max_regression_bps: u128,
    comparisons: &[Comparison],
) {
    let environment = bench_environment();
    let inputs = bench_inputs();
    println!("# {BENCHMARK_SUITE} Comparison");
    println!();
    println!("- Schema: `{REPORT_SCHEMA}`");
    println!("- Version: `{REPORT_VERSION}`");
    println!("- Baseline: `{baseline_path}`");
    println!(
        "- Max regression threshold: `{}%`",
        format_bps(max_regression_bps)
    );
    println!("- Status: `{}`", comparison_status(comparisons));
    println!();
    print_environment_markdown(&environment);
    println!();
    print_inputs_markdown(&inputs);
    println!();
    println!("| Benchmark | Baseline ns | Current ns | Direction | Change | Status |");
    println!("| --- | ---: | ---: | --- | ---: | --- |");
    for comparison in comparisons {
        println!(
            "| `{}` | {} | {} | {} | {}% | {} |",
            comparison.name,
            comparison.baseline_elapsed_ns,
            comparison.current_elapsed_ns,
            comparison.direction.as_str(),
            format_bps(comparison.change_bps),
            if comparison.failed { "fail" } else { "pass" }
        );
    }
}

fn comparison_status(comparisons: &[Comparison]) -> &'static str {
    if comparisons.iter().any(|comparison| comparison.failed) {
        "fail"
    } else {
        "pass"
    }
}

fn json_u64(value: u128) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn parse_percent_to_bps(input: &str) -> Result<u128, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty regression threshold".to_owned());
    }

    let (whole, fraction) = trimmed.split_once('.').unwrap_or((trimmed, ""));
    if whole.is_empty() || !whole.chars().all(|character| character.is_ascii_digit()) {
        return Err(format!("invalid regression threshold: {input}"));
    }
    if fraction.len() > 2 || !fraction.chars().all(|character| character.is_ascii_digit()) {
        return Err(format!(
            "regression threshold must use at most two decimal places: {input}"
        ));
    }

    let whole_bps = whole
        .parse::<u128>()
        .map_err(|error| format!("invalid regression threshold {input}: {error}"))?
        .checked_mul(100)
        .ok_or_else(|| format!("regression threshold is too large: {input}"))?;

    let mut fraction_digits = fraction.to_owned();
    while fraction_digits.len() < 2 {
        fraction_digits.push('0');
    }
    let fraction_bps = if fraction_digits.is_empty() {
        0
    } else {
        fraction_digits
            .parse::<u128>()
            .map_err(|error| format!("invalid regression threshold {input}: {error}"))?
    };

    whole_bps
        .checked_add(fraction_bps)
        .ok_or_else(|| format!("regression threshold is too large: {input}"))
}

fn format_bps(bps: u128) -> String {
    format!("{}.{:02}", bps / 100, bps % 100)
}

impl ChangeDirection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Improvement => "improvement",
            Self::Regression => "regression",
            Self::Unchanged => "unchanged",
        }
    }
}

fn print_markdown(results: &[BenchResult]) {
    let environment = bench_environment();
    let inputs = bench_inputs();
    println!("# {BENCHMARK_SUITE} Report");
    println!();
    println!("- Schema: `{REPORT_SCHEMA}`");
    println!("- Version: `{REPORT_VERSION}`");
    println!("- Iterations per benchmark: `{ITERATIONS}`");
    println!();
    print_environment_markdown(&environment);
    println!();
    print_inputs_markdown(&inputs);
    println!();
    println!("| Benchmark | Iterations | Elapsed ns | Checksum |");
    println!("| --- | ---: | ---: | ---: |");
    for result in results {
        println!(
            "| `{}` | {} | {} | {} |",
            result.name, result.iterations, result.elapsed_ns, result.checksum
        );
    }
}

fn print_environment_markdown(environment: &BenchEnvironment) {
    println!("## Environment");
    println!();
    println!("- rustc: `{}`", environment.rustc_version);
    println!("- target: `{}`", environment.target_triple);
    println!("- features: `{}`", environment.feature_set);
    println!("- commit: `{}`", environment.commit_sha);
}

fn print_inputs_markdown(inputs: &BenchInputs) {
    println!("## Inputs");
    println!();
    println!("- baseline source: `{}`", inputs.baseline_source);
    println!(
        "- max regression threshold: `{}%`",
        inputs.max_regression_percent
    );
    println!("- run mode: `{}`", inputs.run_mode);
}

fn bench_environment() -> BenchEnvironment {
    BenchEnvironment {
        rustc_version: env::var("VSTR_BENCH_RUSTC_VERSION")
            .unwrap_or_else(|_| UNKNOWN_ENV.to_owned()),
        target_triple: env::var("VSTR_BENCH_TARGET_TRIPLE")
            .unwrap_or_else(|_| UNKNOWN_ENV.to_owned()),
        feature_set: active_feature_set(),
        commit_sha: env::var("VSTR_BENCH_COMMIT_SHA").unwrap_or_else(|_| UNKNOWN_ENV.to_owned()),
    }
}

fn bench_inputs() -> BenchInputs {
    BenchInputs {
        baseline_source: benchmark_baseline_source(),
        max_regression_percent: env::var("VSTR_BENCH_MAX_REGRESSION_PCT")
            .unwrap_or_else(|_| "20.00".to_owned()),
        run_mode: env::var("VSTR_BENCH_RUN_MODE").unwrap_or_else(|_| "report".to_owned()),
    }
}

fn benchmark_baseline_source() -> String {
    env::var("VSTR_BENCH_BASE_REF")
        .ok()
        .filter(|value| !value.is_empty())
        .map(|value| format!("git-ref:{value}"))
        .or_else(|| {
            env::var("VSTR_BENCH_BASELINE_JSON")
                .ok()
                .filter(|value| !value.is_empty())
                .map(|value| format!("json:{value}"))
        })
        .unwrap_or_else(|| "none".to_owned())
}

fn active_feature_set() -> String {
    let mut features = Vec::new();

    if cfg!(feature = "matcher-aho-corasick") {
        features.push("matcher-aho-corasick");
    }
    if cfg!(feature = "pattern-regex") {
        features.push("pattern-regex");
    }
    if cfg!(feature = "unicode-normalization") {
        features.push("unicode-normalization");
    }
    if cfg!(feature = "unicode-segmentation") {
        features.push("unicode-segmentation");
    }
    if cfg!(feature = "unicode-width") {
        features.push("unicode-width");
    }

    if features.is_empty() {
        "default".to_owned()
    } else {
        features.join(",")
    }
}

fn json_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn bench_contains(input: &str) -> usize {
    usize::from(knifer_rs::vstr::contains(input, "rust"))
        + usize::from(knifer_rs::vstr::contains_any(input, ["java", "rust"]))
}

fn bench_find_all(input: &str) -> usize {
    knifer_rs::vstr::find_all(input, "rust").len()
        + knifer_rs::vstr::find_all_ignore_case(input, "knifer-rs").len()
}

fn bench_replace_many(input: &str) -> usize {
    knifer_rs::vstr::replace_many(input, [("rust", "rs"), ("world", "team")]).len()
}

fn bench_to_snake_case(input: &str) -> usize {
    knifer_rs::vstr::to_snake_case(input).len()
}

fn bench_wrap(input: &str) -> usize {
    knifer_rs::vstr::wrap(input, 80).len()
}

fn bench_levenshtein(input: &str) -> usize {
    let left = knifer_rs::vstr::truncate(input, 64);
    knifer_rs::vstr::levenshtein_distance(left, "Knifer-RS hello rust utility")
}

fn bench_ant_path(path: &str) -> usize {
    usize::from(knifer_rs::vstr::ant_path_match("/api/**/users/?*", path))
}
