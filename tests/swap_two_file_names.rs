mod test;

const TEST_FILE_1: &'static str = "file1";
const TEST_FILE_2: &'static str = "file2";

const TEST_FILE_1_CONTENT: &'static str = "file1 test content";
const TEST_FILE_2_CONTENT: &'static str = "file2 test content";

#[test]
fn swap_two_file_names() {
    let test_name = "swap_two_file_names";

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
            to: TEST_FILE_1,
        },
    ];
    let args = vec!["-v", "."];

    test::init(test_name, files);

    // when
    let output = test::bulkmv_stdout(test_name, args, renames);

    // then
    assert_eq!(
        output.trim(),
        format!("{TEST_FILE_1} -> {TEST_FILE_2}\n{TEST_FILE_2} -> {TEST_FILE_1}")
    );

    let file1_content = test::get_file_content(test_name, TEST_FILE_2);
    assert_eq!(file1_content, TEST_FILE_2_CONTENT);

    let file2_content = test::get_file_content(test_name, TEST_FILE_2);
    assert_eq!(file2_content, TEST_FILE_1_CONTENT);
}
