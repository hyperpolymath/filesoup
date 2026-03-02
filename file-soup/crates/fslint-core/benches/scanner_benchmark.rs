use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fslint_core::{Config, PluginLoader, Scanner, ScannerConfig};
use std::fs;
use tempfile::TempDir;

fn create_test_files(dir: &std::path::Path, count: usize) {
    for i in 0..count {
        let file_path = dir.join(format!("test_{}.txt", i));
        fs::write(file_path, format!("Test content {}", i)).unwrap();
    }
}

fn create_nested_structure(dir: &std::path::Path, depth: usize, files_per_dir: usize) {
    create_test_files(dir, files_per_dir);

    if depth > 0 {
        for i in 0..3 {
            let subdir = dir.join(format!("subdir_{}", i));
            fs::create_dir(&subdir).unwrap();
            create_nested_structure(&subdir, depth - 1, files_per_dir);
        }
    }
}

fn bench_scanner_small(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(temp_dir.path(), 10);

    let config = ScannerConfig::default();
    let loader = PluginLoader::new();

    c.bench_function("scan_10_files", |b| {
        b.iter(|| {
            let mut scanner = Scanner::new(config.clone(), loader.clone());
            black_box(scanner.scan(temp_dir.path()).unwrap());
        });
    });
}

fn bench_scanner_medium(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(temp_dir.path(), 100);

    let config = ScannerConfig::default();
    let loader = PluginLoader::new();

    c.bench_function("scan_100_files", |b| {
        b.iter(|| {
            let mut scanner = Scanner::new(config.clone(), loader.clone());
            black_box(scanner.scan(temp_dir.path()).unwrap());
        });
    });
}

fn bench_scanner_large(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(temp_dir.path(), 1000);

    let config = ScannerConfig::default();
    let loader = PluginLoader::new();

    c.bench_function("scan_1000_files", |b| {
        b.iter(|| {
            let mut scanner = Scanner::new(config.clone(), loader.clone());
            black_box(scanner.scan(temp_dir.path()).unwrap());
        });
    });
}

fn bench_scanner_nested(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_nested_structure(temp_dir.path(), 3, 10);

    let config = ScannerConfig::default();
    let loader = PluginLoader::new();

    c.bench_function("scan_nested_structure", |b| {
        b.iter(|| {
            let mut scanner = Scanner::new(config.clone(), loader.clone());
            black_box(scanner.scan(temp_dir.path()).unwrap());
        });
    });
}

fn bench_scanner_with_cache(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_test_files(temp_dir.path(), 100);

    let config = ScannerConfig::default();
    let loader = PluginLoader::new();
    let mut scanner = Scanner::new(config, loader);

    // First scan to populate cache
    scanner.scan(temp_dir.path()).unwrap();

    c.bench_function("scan_with_cache", |b| {
        b.iter(|| {
            black_box(scanner.scan(temp_dir.path()).unwrap());
        });
    });
}

fn bench_max_depth(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    create_nested_structure(temp_dir.path(), 5, 5);

    let loader = PluginLoader::new();

    let mut group = c.benchmark_group("max_depth");
    for depth in [1, 3, 5, 10].iter() {
        let mut config = ScannerConfig::default();
        config.max_depth = Some(*depth);

        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, _| {
            b.iter(|| {
                let mut scanner = Scanner::new(config.clone(), loader.clone());
                black_box(scanner.scan(temp_dir.path()).unwrap());
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_scanner_small,
    bench_scanner_medium,
    bench_scanner_large,
    bench_scanner_nested,
    bench_scanner_with_cache,
    bench_max_depth
);
criterion_main!(benches);
