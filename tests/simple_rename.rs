use std::{env, fs, path, process, str};

const TESTDATA_DIRNAME: &'static str = "testdata";
const TEST_FILE_NAME: &'static str = "./file1";
const TEST_FILE_NAME_RENAMED: &'static str = "./file2";

#[test]
fn rename_one_file() {
    let test_dir = get_test_dir("rename_one_file");
    clear_test_dir(&test_dir);
    make_test_file(&test_dir);
    set_working_directory(&test_dir);
    let tested_executable = get_tested_executable();

    let status = process::Command::new(tested_executable)
        .env(
            "EDITOR",
            format!(
                "{} {} {}",
                env!("CARGO_BIN_EXE_mockedit"),
                TEST_FILE_NAME,
                TEST_FILE_NAME_RENAMED
            ),
        )
        .arg(".")
        .status()
        .unwrap();

    assert!(status.success());

    let files = get_dir_items(".");
    assert_eq!(files, vec![TEST_FILE_NAME_RENAMED]);
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
    fs::write(test_file_path, "test data").unwrap();
}

fn set_working_directory(test_dir: &path::PathBuf) {
    env::set_current_dir(test_dir).unwrap();
}

fn get_tested_executable() -> String {
    env!("CARGO_BIN_EXE_bulkmv").to_string()
}

fn get_dir_items(path: &str) -> Vec<String> {
    let dir = fs::read_dir(path).unwrap();

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap()).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    map_dir_entries_to_strings(&dir_contents)
}

fn map_dir_entries_to_strings(dir_items: &Vec<fs::DirEntry>) -> Vec<String> {
    dir_items
        .iter()
        .map(|dir_entry| dir_entry.path().to_string_lossy().into_owned())
        .collect()
}
