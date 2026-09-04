#![allow(dead_code)]
use std::{env, fs, path, process, str};

const TESTDATA_DIRNAME: &'static str = "testdata";

pub(crate) fn init(test_name: &str, files_to_create: Vec<File>) {
    let test_dir = get_test_dir(test_name);
    clear_test_dir(&test_dir);
    files_to_create
        .iter()
        .for_each(|file| make_test_file(&test_dir, file.name, file.content));
    set_working_directory(&test_dir);
}

pub(crate) struct File<'a> {
    pub(crate) name: &'a str,
    pub(crate) content: &'a str,
}

fn get_test_dir(test_dir_name: &str) -> path::PathBuf {
    let tmpdir = env!("CARGO_TARGET_TMPDIR");
    path::PathBuf::from(tmpdir)
        .join(TESTDATA_DIRNAME)
        .join(test_dir_name)
}

fn clear_test_dir(test_dir: &path::PathBuf) {
    if !fs::exists(test_dir).unwrap() {
        return;
    }
    fs::remove_dir_all(test_dir).unwrap();
}

pub(crate) fn make_test_file(test_dir: &path::PathBuf, filename: &str, file_content: &str) {
    let test_file_path = test_dir.join(filename);
    fs::create_dir_all(test_dir).unwrap();
    fs::write(test_file_path, file_content).unwrap();
}

fn set_working_directory(test_dir: &path::PathBuf) {
    env::set_current_dir(test_dir).unwrap();
}

pub(crate) fn bulkmv(args: Vec<&str>, mockedit_renames: Vec<Rename>) {
    let output = run_bulkmv(args, mockedit_renames);
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("stdout: {}\nstderr: {}", stdout, stderr);
    }
}

pub(crate) fn bulkmv_stdout(args: Vec<&str>, mockedit_renames: Vec<Rename>) -> String {
    let output = run_bulkmv(args, mockedit_renames);
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("stdout: {}\nstderr: {}", stdout, stderr);
    }
    stdout
}

fn run_bulkmv(args: Vec<&str>, mockedit_renames: Vec<Rename>) -> process::Output {
    let tested_executable = env!("CARGO_BIN_EXE_bulkmv");
    process::Command::new(tested_executable)
        .env("EDITOR", mockedit(mockedit_renames))
        .args(args)
        .output()
        .unwrap()
}

pub(crate) struct Rename<'a> {
    pub(crate) from: &'a str,
    pub(crate) to: &'a str,
}

fn mockedit(renames: Vec<Rename>) -> String {
    let mut buf: String = String::new();
    buf.push_str(env!("CARGO_BIN_EXE_mockedit"));
    renames.iter().for_each(|r| {
        buf.push_str(" ");
        buf.push_str(r.from);
        buf.push_str(" ");
        buf.push_str(r.to);
    });
    buf
}

pub(crate) fn get_dir_items(path: &str) -> Vec<String> {
    let dir = fs::read_dir(path).unwrap();

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap()).collect();
    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_string_lossy().to_string())
        .collect()
}

pub(crate) fn get_file_content(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
