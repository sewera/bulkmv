mod test;

const TEST_FILE_NAME: &'static str = "file1";
const TEST_FILE_NAME_RENAMED: &'static str = "file2";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

#[test]
fn quiet_output() {
    // given
    let test_name = "quiet_output";
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

#[test]
fn verbose_output_param_before_dir() {
    // given
    let test_name = "verbose_output_param_before_dir";
    let files = vec![test::File {
        name: TEST_FILE_NAME,
        content: TEST_FILE_CONTENT,
    }];
    let renames = vec![test::Rename {
        from: TEST_FILE_NAME,
        to: TEST_FILE_NAME_RENAMED,
    }];
    test::init(test_name, files);
    let args = vec!["-v", "."];

    // when
    let output = test::bulkmv_stdout(args, renames);

    // then
    assert_eq!(
        output.trim(),
        format!("{TEST_FILE_NAME} -> {TEST_FILE_NAME_RENAMED}\n")
    );
}

#[test]
fn verbose_output_param_after_dir() {
    // given
    let test_name = "verbose_output_param_after_dir";
    let files = vec![test::File {
        name: TEST_FILE_NAME,
        content: TEST_FILE_CONTENT,
    }];
    let renames = vec![test::Rename {
        from: TEST_FILE_NAME,
        to: TEST_FILE_NAME_RENAMED,
    }];
    test::init(test_name, files);
    let args = vec![".", "-v"];

    // when
    let output = test::bulkmv_stdout(args, renames);

    // then
    assert_eq!(
        output.trim(),
        format!("{TEST_FILE_NAME} -> {TEST_FILE_NAME_RENAMED}\n")
    );
}
