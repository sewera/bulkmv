mod test;

const TEST_FILE_1: &'static str = "file1";
const TEST_FILE_2: &'static str = "file2";

const TEST_FILE_1_CONTENT: &'static str = "file1 test content";
const TEST_FILE_2_CONTENT: &'static str = "file2 test content";

#[test]
fn duplicate_target_names() {
    let test_name = "duplicate_target_names";

    let files = vec![
        test::File {
            name: TEST_FILE_1,
            content: TEST_FILE_1_CONTENT,
        },
        test::File {
            name: TEST_FILE_2,
            content: TEST_FILE_2_CONTENT,
        },
    ];
    let renames = vec![
        test::Rename {
            from: TEST_FILE_1,
            to: TEST_FILE_2,
        },
        test::Rename {
            from: TEST_FILE_2,
            to: TEST_FILE_2,
        },
    ];
    let args = vec!["-v", "."];

    test::init(test_name, files);

    // when
    let output = test::bulkmv_stderr_fail(test_name, args, renames);

    // then
    assert!(output.contains(format!("error: duplicate target names: {TEST_FILE_2}").as_str()));
}
