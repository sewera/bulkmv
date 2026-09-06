#![allow(dead_code)]
use std::{env, fs, path, process, str};

const TESTDATA_DIRNAME: &'static str = "testdata";

pub fn init(test_name: &str, files_to_create: Vec<File>) {
    let test_dir = get_test_dir(test_name);
    clear_test_dir(&test_dir);
    files_to_create
        .iter()
        .for_each(|file| make_test_file(&test_dir, file.name, file.content));
}

pub struct File<'a> {
    pub name: &'a str,
    pub content: &'a str,
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

pub fn make_test_file(test_dir: &path::PathBuf, filename: &str, file_content: &str) {
    let test_file_path = test_dir.join(filename);
    fs::create_dir_all(test_dir).unwrap();
    fs::write(test_file_path, file_content).unwrap();
}

pub fn bulkmv(test_name: &str, args: Vec<&str>, mockedit_renames: Vec<Rename>) {
    let output = run_bulkmv(test_name, args, mockedit_renames);
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("stdout: {}\nstderr: {}", stdout, stderr);
    }
}

pub fn bulkmv_stdout(test_name: &str, args: Vec<&str>, mockedit_renames: Vec<Rename>) -> String {
    let output = run_bulkmv(test_name, args, mockedit_renames);
    let stdout = String::from_utf8(output.stdout).unwrap();
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("stdout: {}\nstderr: {}", stdout, stderr);
    }
    stdout
}

fn run_bulkmv(test_name: &str, args: Vec<&str>, mockedit_renames: Vec<Rename>) -> process::Output {
    let tested_executable = env!("CARGO_BIN_EXE_bulkmv");
    process::Command::new(tested_executable)
        .env("BULKMV_CWD", get_test_dir(test_name))
        .env("EDITOR", mockedit(mockedit_renames))
        .args(args)
        .output()
        .unwrap()
}

pub struct Rename<'a> {
    pub from: &'a str,
    pub to: &'a str,
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

pub fn get_dir_items(test_name: &str) -> Vec<String> {
    let path = get_test_dir(test_name);
    let dir = fs::read_dir(path).unwrap();

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap()).collect();
    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_string_lossy().to_string())
        .collect()
}

pub fn get_file_content(test_name: &str, path: &str) -> String {
    let dir = get_test_dir(test_name);
    let file = path::PathBuf::from(dir).join(path);
    fs::read_to_string(file).unwrap()
}
