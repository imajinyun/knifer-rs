//! Smoke benchmark coverage for representative `vstr` helpers.

use std::hint::black_box;
use std::time::Instant;

fn main() {
    let sample = "Knifer-RS hello rust world 你好 ".repeat(256);
    let path = "/api/v1/projects/knifer-rs/users/42";

    run_case("contains", &sample, bench_contains);
    run_case("find_all", &sample, bench_find_all);
    run_case("replace_many", &sample, bench_replace_many);
    run_case("to_snake_case", &sample, bench_to_snake_case);
    run_case("wrap", &sample, bench_wrap);
    run_case("levenshtein_distance", &sample, bench_levenshtein);
    run_case("ant_path_match", path, bench_ant_path);
}

fn run_case(name: &str, input: &str, case: fn(&str) -> usize) {
    let start = Instant::now();
    let mut checksum = 0usize;
    for _ in 0..128 {
        checksum = checksum.wrapping_add(case(black_box(input)));
    }
    println!("{name}: {:?} checksum={checksum}", start.elapsed());
}

fn bench_contains(input: &str) -> usize {
    usize::from(kniferrs::vstr::contains(input, "rust"))
        + usize::from(kniferrs::vstr::contains_any(input, ["java", "rust"]))
}

fn bench_find_all(input: &str) -> usize {
    kniferrs::vstr::find_all(input, "rust").len()
        + kniferrs::vstr::find_all_ignore_case(input, "knifer-rs").len()
}

fn bench_replace_many(input: &str) -> usize {
    kniferrs::vstr::replace_many(input, [("rust", "rs"), ("world", "team")]).len()
}

fn bench_to_snake_case(input: &str) -> usize {
    kniferrs::vstr::to_snake_case(input).len()
}

fn bench_wrap(input: &str) -> usize {
    kniferrs::vstr::wrap(input, 80).len()
}

fn bench_levenshtein(input: &str) -> usize {
    let left = kniferrs::vstr::truncate(input, 64);
    kniferrs::vstr::levenshtein_distance(left, "Knifer-RS hello rust utility")
}

fn bench_ant_path(path: &str) -> usize {
    usize::from(kniferrs::vstr::ant_path_match("/api/**/users/?*", path))
}
