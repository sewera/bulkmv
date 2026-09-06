mod test;

const TEST_FILE_NAME: &'static str = "file1";
const TEST_FILE_NAME_RENAMED: &'static str = "file2";

const TEST_NOT_RENAMED_FILE_NAME: &'static str = "file_not_renamed";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

#[test]
fn param_before_dir() {
    let test_name = "verbose_output.param_before_dir";

    let files = vec![
        test::File {
            name: TEST_FILE_NAME,
            content: TEST_FILE_CONTENT,
        },
        test::File {
            name: TEST_NOT_RENAMED_FILE_NAME,
            content: TEST_FILE_CONTENT,
        },
    ];
    let renames = vec![test::Rename {
        from: TEST_FILE_NAME,
        to: TEST_FILE_NAME_RENAMED,
    }];
    let args = vec!["-v", "."];

    test::init(test_name, files);

    // when
    let output = test::bulkmv_stdout(test_name, args, renames);

    // then
    assert_eq!(
        output.trim(),
        format!("{TEST_FILE_NAME} -> {TEST_FILE_NAME_RENAMED}")
    );
}
