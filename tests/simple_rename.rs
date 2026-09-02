use std::{env, fs, path, process, str};

const TESTDATA_DIRNAME: &'static str = "testdata";
const TEST_FILE_NAME: &'static str = "./file1";
const TEST_FILE_NAME_RENAMED: &'static str = "./file2";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

#[test]
fn rename_one_file() {
    let test_name = "rename_one_file";
    init_test(test_name);

    run_bulkmv();
    let files = get_dir_items(".");
    assert_eq!(files, vec![TEST_FILE_NAME_RENAMED]);
}

fn init_test(test_name: &str) {
    let test_dir = get_test_dir(test_name);
    clear_test_dir(&test_dir);
    make_test_file(&test_dir);
    set_working_directory(&test_dir);
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

fn make_test_file(test_dir: &path::PathBuf) {
    let test_file_path = test_dir.join(TEST_FILE_NAME);
    fs::create_dir_all(test_dir).unwrap();
    fs::write(test_file_path, TEST_FILE_CONTENT).unwrap();
}

fn set_working_directory(test_dir: &path::PathBuf) {
    env::set_current_dir(test_dir).unwrap();
}

fn run_bulkmv() {
    let tested_executable = env!("CARGO_BIN_EXE_bulkmv");
    let output = process::Command::new(tested_executable)
        .env(
            "EDITOR",
            mockedit(vec![Rename {
                from: TEST_FILE_NAME,
                to: TEST_FILE_NAME_RENAMED,
            }]),
        )
        .arg(".")
        .output()
        .unwrap();
    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("stdout: {}\nstderr: {}", stdout, stderr);
    }
}

struct Rename<'a> {
    from: &'a str,
    to: &'a str,
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

fn get_dir_items(path: &str) -> Vec<String> {
    let dir = fs::read_dir(path).unwrap();

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap()).collect();
    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
        .iter()
        .map(|dir_entry| dir_entry.path().to_string_lossy().to_string())
        .collect()
}
