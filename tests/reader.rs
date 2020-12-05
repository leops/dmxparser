#![feature(test)]

extern crate test;

use std::{
    env,
    fs::{read_dir, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use test::{test_main, ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};

use serde_json::Value;

use dmxparser::{formats::vmap::read_vmap, from_reader, from_slice, serde::from_file};

fn main() {
    let mut args = Vec::new();
    let mut fixtures_path = PathBuf::from("tests/fixtures");

    let mut args_iter = env::args();
    while let Some(arg) = args_iter.next() {
        if arg == "--fixtures" {
            let path = args_iter.next().expect("--fixtures expects a value");
            fixtures_path = PathBuf::from(path);
        } else {
            args.push(arg);
        }
    }

    let mut tests = Vec::new();

    visit_directory(&fixtures_path, &mut |path_1| {
        let path_2 = path_1.clone();
        let path_3 = path_1.clone();
        let path_4 = path_1.clone();
        let path_5 = path_1.clone();
        let path_6 = path_1.clone();

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!("from_slice({})", path_1.display())),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let mut data = Vec::new();
                let mut file = File::open(path_1).unwrap();
                file.read_to_end(&mut data).unwrap();

                if let Err(err) = from_slice(&data) {
                    panic!("{}", err);
                }
            })),
        });

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!("from_reader({})", path_2.display())),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let file = File::open(path_2).unwrap();
                let reader = BufReader::new(file);

                if let Err(err) = from_reader(reader) {
                    panic!("{}", err);
                }
            })),
        });

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!("from_file(from_slice({}))", path_3.display())),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let mut data = Vec::new();
                let mut file = File::open(path_3).unwrap();
                file.read_to_end(&mut data).unwrap();

                let file = from_slice(&data).unwrap();

                if let Err(err) = from_file::<_, _, Value>(&file) {
                    panic!("{}", err);
                }
            })),
        });

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!(
                    "from_file(from_reader({}))",
                    path_4.display()
                )),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let file = File::open(path_4).unwrap();
                let reader = BufReader::new(file);
                let file = from_reader(reader).unwrap();

                if let Err(err) = from_file::<Vec<u8>, String, Value>(&file) {
                    panic!("{}", err);
                }
            })),
        });

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!("read_vmap(from_slice({}))", path_5.display())),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let mut data = Vec::new();
                let mut file = File::open(path_5).unwrap();
                file.read_to_end(&mut data).unwrap();

                let file = from_slice(&data).unwrap();

                if let Err(err) = read_vmap(&file) {
                    panic!("{}", err);
                }
            })),
        });

        tests.push(TestDescAndFn {
            desc: TestDesc {
                name: TestName::DynTestName(format!(
                    "read_vmap(from_reader({}))",
                    path_6.display()
                )),
                ignore: false,
                should_panic: ShouldPanic::No,
                allow_fail: false,
                test_type: TestType::IntegrationTest,
            },
            testfn: TestFn::DynTestFn(Box::new(move || {
                let file = File::open(path_6).unwrap();
                let reader = BufReader::new(file);
                let file = from_reader(reader).unwrap();

                if let Err(err) = read_vmap(&file) {
                    panic!("{}", err);
                }
            })),
        });
    });

    test_main(&args, tests, None);
}

/// Recursively traverses a directory, yielding all *.vmap files to the visitor as they are found
fn visit_directory(dir: &Path, handle_file: &mut impl FnMut(PathBuf)) {
    let iter = match read_dir(dir) {
        Ok(iter) => iter,
        Err(err) => {
            eprintln!("error reading {}: {}", dir.display(), err);
            return;
        }
    };

    for entry in iter {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("error reading entry in {}: {}", dir.display(), err);
                continue;
            }
        };

        let path = entry.path();
        let kind = match entry.file_type() {
            Ok(kind) => kind,
            Err(err) => {
                eprintln!("error reading file type of {}: {}", path.display(), err);
                continue;
            }
        };

        if kind.is_dir() {
            visit_directory(&path, handle_file);
        }

        if let Some(ext) = path.extension() {
            if ext == "vmap" {
                handle_file(path);
            }
        }
    }
}
