mod test;

const TEST_FILE_NAME: &'static str = "file1";
const TEST_FILE_NAME_RENAMED: &'static str = "file1_renamed";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

const TEST_FILE_NAME_2: &'static str = "file2";
const TEST_FILE_NAME_2_RENAMED: &'static str = "file2_renamed";

const TEST_FILE_2_CONTENT: &'static str = "file2 test content";

#[test]
fn rename_one_file() {
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
    test::bulkmv(test_name, args, renames);

    // then
    let files = test::get_dir_items(test_name);
    assert_eq!(files, vec![TEST_FILE_NAME_RENAMED]);

    let file_content = test::get_file_content(test_name, TEST_FILE_NAME_RENAMED);
    assert_eq!(file_content, TEST_FILE_CONTENT);
}

#[test]
fn rename_multiple_files() {
    // given
    let test_name = "rename_multiple_files";
    let files = vec![
        test::File {
            name: TEST_FILE_NAME,
            content: TEST_FILE_CONTENT,
        },
        test::File {
            name: TEST_FILE_NAME_2,
            content: TEST_FILE_2_CONTENT,
        },
    ];
    let renames = vec![
        test::Rename {
            from: TEST_FILE_NAME,
            to: TEST_FILE_NAME_RENAMED,
        },
        test::Rename {
            from: TEST_FILE_NAME_2,
            to: TEST_FILE_NAME_2_RENAMED,
        },
    ];
    test::init(test_name, files);
    let args = vec!["."];

    // when
    test::bulkmv(test_name, args, renames);

    // then
    let files = test::get_dir_items(test_name);
    assert_eq!(
        files,
        vec![TEST_FILE_NAME_RENAMED, TEST_FILE_NAME_2_RENAMED]
    );

    let file1_content = test::get_file_content(test_name, TEST_FILE_NAME_RENAMED);
    assert_eq!(file1_content, TEST_FILE_CONTENT);
    let file2_content = test::get_file_content(test_name, TEST_FILE_NAME_2_RENAMED);
    assert_eq!(file2_content, TEST_FILE_2_CONTENT);
}
