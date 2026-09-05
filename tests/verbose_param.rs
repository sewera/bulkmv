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
    let output = test::bulkmv_stdout(test_name, args, renames);

    // then
    assert_eq!(output.trim(), "");
}

mod verbose_output {
    use super::*;

    #[test]
    fn param_before_dir() {
        test_verbose_output("verbose_output.param_before_dir", vec!["-v", "."]);
    }

    #[test]
    fn param_after_dir() {
        test_verbose_output("verbose_output.param_after_dir", vec![".", "-v"]);
    }

    #[test]
    fn dir_after_separator() {
        test_verbose_output("verbose_output.dir_after_separator", vec!["-v", "--", "."]);
    }

    #[test]
    fn long_flag_before_dir() {
        test_verbose_output(
            "verbose_output.long_flag_before_dir",
            vec!["--verbose", "."],
        );
    }

    #[test]
    fn long_flag_after_dir() {
        test_verbose_output("verbose_output.long_flag_after_dir", vec![".", "--verbose"]);
    }

    #[test]
    fn long_flag_and_dir_after_separator() {
        test_verbose_output(
            "verbose_output.long_flag_and_dir_after_separator",
            vec!["--verbose", "--", "."],
        );
    }

    fn test_verbose_output(test_name: &str, args: Vec<&str>) {
        // given
        let files = vec![test::File {
            name: TEST_FILE_NAME,
            content: TEST_FILE_CONTENT,
        }];
        let renames = vec![test::Rename {
            from: TEST_FILE_NAME,
            to: TEST_FILE_NAME_RENAMED,
        }];
        test::init(test_name, files);

        // when
        let output = test::bulkmv_stdout(test_name, args, renames);

        // then
        assert_eq!(
            output.trim(),
            format!("{TEST_FILE_NAME} -> {TEST_FILE_NAME_RENAMED}")
        );
    }
}
