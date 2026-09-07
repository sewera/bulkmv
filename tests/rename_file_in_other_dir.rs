mod test;

const SUBDIRECTORY: &'static str = "subdirectory";

const TEST_FILE_NAME: &'static str = "file1";
const TEST_FILE_NAME_RENAMED: &'static str = "file1_renamed";

const TEST_FILE_CONTENT: &'static str = "file1 test content";

#[test]
fn rename_file_in_subdirectory() {
    // given
    let test_name = "rename_file_in_subdirectory";

    let file_name = format!("{SUBDIRECTORY}/{TEST_FILE_NAME}");
    let files = vec![test::File {
        name: file_name.as_str(),
        content: TEST_FILE_CONTENT,
    }];

    let target_file_name = format!("{SUBDIRECTORY}/{TEST_FILE_NAME_RENAMED}");
    let renames = vec![test::Rename {
        from: file_name.as_str(),
        to: target_file_name.as_str(),
    }];

    test::init_with_subdirectory(test_name, SUBDIRECTORY, files);
    let args = vec![SUBDIRECTORY];

    // when
    test::bulkmv(test_name, args, renames);

    // then
    let files = test::get_dir_items_in_subdirectory(test_name, SUBDIRECTORY);
    assert_eq!(files, vec![TEST_FILE_NAME_RENAMED]);

    let file_content = test::get_file_content(test_name, target_file_name.as_str());
    assert_eq!(file_content, TEST_FILE_CONTENT);
}
