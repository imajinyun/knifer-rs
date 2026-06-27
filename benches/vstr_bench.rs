//! Stable benchmark suite for representative `vstr` helpers.

use std::env;
use std::fmt::Write as _;
use std::hint::black_box;
use std::time::{Duration, Instant};

const ITERATIONS: usize = 1_024;
const REPORT_VERSION: &str = "v1";
const BENCHMARK_SUITE: &str = "vstr_bench";

#[derive(Clone, Copy)]
struct BenchResult {
    name: &'static str,
    iterations: usize,
    elapsed_ns: u128,
    checksum: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ReportFormat {
    Plain,
    Json,
    Markdown,
}

fn main() {
    let sample = "Knifer-RS hello rust world 你好 ".repeat(512);
    let path = "/api/v1/projects/knifer-rs/users/42";
    let format = report_format();

    let results = [
        run_case("contains", &sample, bench_contains),
        run_case("find_all", &sample, bench_find_all),
        run_case("replace_many", &sample, bench_replace_many),
        run_case("to_snake_case", &sample, bench_to_snake_case),
        run_case("wrap", &sample, bench_wrap),
        run_case("levenshtein_distance", &sample, bench_levenshtein),
        run_case("ant_path_match", path, bench_ant_path),
    ];

    match format {
        ReportFormat::Plain => print_plain(&results),
        ReportFormat::Json => print_json(&results),
        ReportFormat::Markdown => print_markdown(&results),
    }
}

fn report_format() -> ReportFormat {
    match env::args().nth(1).as_deref() {
        Some("--json") => ReportFormat::Json,
        Some("--markdown") => ReportFormat::Markdown,
        _ => ReportFormat::Plain,
    }
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
    for result in results {
        println!(
            "bench={} iterations={} elapsed_ns={} checksum={}",
            result.name, result.iterations, result.elapsed_ns, result.checksum
        );
    }
}

fn print_json(results: &[BenchResult]) {
    let mut output = String::new();
    write!(
        output,
        "{{\"suite\":\"{BENCHMARK_SUITE}\",\"version\":\"{REPORT_VERSION}\",\"results\":["
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

fn print_markdown(results: &[BenchResult]) {
    println!("# {BENCHMARK_SUITE} Report");
    println!();
    println!("- Version: `{REPORT_VERSION}`");
    println!("- Iterations per benchmark: `{ITERATIONS}`");
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
