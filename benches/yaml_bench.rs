use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;

extern crate minimal_yaml;
extern crate yaml_rust;

use std::fs::{File};
use std::io::Read;

criterion_group!(benches, yaml_benchmark, yaml_rust_benchmark, yaml_small_benchmark, yaml_rust_small_benchmark);
criterion_main!(benches);

fn minimal_parse_file(path: impl AsRef<Path>) {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    minimal_yaml::parse(&buf).unwrap();
}

fn rust_parse_file(path: impl AsRef<Path>) {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    yaml_rust::YamlLoader::load_from_str(&buf).unwrap();
}

fn yaml_rust_benchmark(c: &mut Criterion) {
    c.bench_function("large yaml file: yaml-rust", |b| b.iter(|| rust_parse_file("bigfile.yaml")));
}
fn yaml_benchmark(c: &mut Criterion) {
    c.bench_function("large yaml file: minimal-yaml", |b| b.iter(|| minimal_parse_file("bigfile.yaml")));
}

fn yaml_rust_small_benchmark(c: &mut Criterion) {
    c.bench_function("small yaml file: yaml-rust", |b| b.iter(|| rust_parse_file("smallfile.yaml")));
}
fn yaml_small_benchmark(c: &mut Criterion) {
    c.bench_function("small yaml file: minimal-yaml", |b| b.iter(|| minimal_parse_file("smallfile.yaml")));
}

