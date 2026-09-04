mod test;

const TEST_FILE_NAME: &'static str = "file1";
const TEST_FILE_NAME_RENAMED: &'static str = "file2";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

#[test]
fn quiet_output() {
    // given
    let test_name = "rename_one_file";
    let files = vec![test::File {
        name: TEST_FILE_NAME,
        content: TEST_FILE_CONTENT,
    }];
    let renames = vec![test::Rename {
        from: TEST_FILE_NAME,
        to: TEST_FILE_NAME_RENAMED,
    }];
    test::init(test_name, files);
    let args = vec!["."];

    // when
    let output = test::bulkmv_stdout(args, renames);

    // then
    assert_eq!(output.trim(), "");
}
