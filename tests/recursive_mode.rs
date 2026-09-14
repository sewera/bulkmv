mod test;

#[test]
fn move_files_in_recursive_mode() {
    // given
    let test_name = "move_files_in_recursive_mode";

    let subdirectory_1 = "subdirectory_1";
    let subdirectory_2 = "subdirectory_2";

    let file_1_1 = format!("./{}/{}", subdirectory_1, "file_1_1");
    let filename_1_1_renamed = "file_1_1_renamed";
    let file_1_1_renamed = format!("{}/{}", subdirectory_1, filename_1_1_renamed);
    let file_1_1_content = "file_1_1 test content";

    let file_1_2 = format!("./{}/{}", subdirectory_1, "file_1_2");
    let filename_1_2_renamed = "file_1_2_renamed";
    let file_1_2_renamed = format!("{}/{}", subdirectory_2, filename_1_2_renamed);
    let file_1_2_content = "file_1_2 test content";

    let file_2_1 = format!("./{}/{}", subdirectory_2, "file_2_1");
    let filename_2_1_renamed = "file_2_1_renamed";
    let file_2_1_renamed = format!("{}/{}", subdirectory_1, filename_2_1_renamed);
    let file_2_1_content = "file_2_1 test content";

    let files = vec![
        test::File {
            name: file_1_1.as_str(),
            content: file_1_1_content,
        },
        test::File {
            name: file_1_2.as_str(),
            content: file_1_2_content,
        },
        test::File {
            name: file_2_1.as_str(),
            content: file_2_1_content,
        },
    ];

    let renames = vec![
        test::Rename {
            from: file_1_1.as_str(),
            to: file_1_1_renamed.as_str(),
        },
        test::Rename {
            from: file_1_2.as_str(),
            to: file_1_2_renamed.as_str(),
        },
        test::Rename {
            from: file_2_1.as_str(),
            to: file_2_1_renamed.as_str(),
        },
    ];

    test::init_with_subdirectories(
        test_name,
        vec![subdirectory_1.to_string(), subdirectory_2.to_string()],
        files,
    );
    let args = vec!["-r", "."];

    // when
    test::bulkmv(test_name, args, renames);

    // then
    let files_subdirectory_1 = test::get_dir_items_in_subdirectory(test_name, subdirectory_1);
    assert_eq!(
        files_subdirectory_1,
        vec![filename_1_1_renamed, filename_2_1_renamed]
    );
    let files_subdirectory_2 = test::get_dir_items_in_subdirectory(test_name, subdirectory_2);
    assert_eq!(files_subdirectory_2, vec![filename_1_2_renamed]);

    vec![
        (file_1_1_renamed, file_1_1_content),
        (file_1_2_renamed, file_1_2_content),
        (file_2_1_renamed, file_2_1_content),
    ]
    .iter()
    .for_each(|(renamed_file, expected_content)| {
        let actual_content = test::get_file_content(test_name, renamed_file.as_str());
        assert_eq!(actual_content, expected_content.to_string());
    });
}

#[test]
fn move_files_in_recursive_mode_and_automatically_create_subdirectory() {
    // given
    let test_name = "move_files_in_recursive_mode_and_automatically_create_subdirectory";

    let subdirectory_1 = "subdirectory_1";
    let subdirectory_2 = "subdirectory_2";

    let file = format!("./{}/{}", subdirectory_1, "file");
    let filename_renamed = "file_renamed";
    let file_renamed = format!("{}/{}", subdirectory_2, filename_renamed);
    let file_content = "file test content";

    let files = vec![test::File {
        name: file.as_str(),
        content: file_content,
    }];

    let renames = vec![test::Rename {
        from: file.as_str(),
        to: file_renamed.as_str(),
    }];

    test::init_with_subdirectories(test_name, vec![subdirectory_1.to_string()], files);
    let args = vec!["-rp", "."];

    // when
    test::bulkmv(test_name, args, renames);

    // then
    let files_subdirectory_2 = test::get_dir_items_in_subdirectory(test_name, subdirectory_2);
    assert_eq!(files_subdirectory_2, vec![filename_renamed]);

    let subdirectories = test::get_dir_items(test_name);
    assert_eq!(subdirectories, vec![subdirectory_1, subdirectory_2]);

    let actual_content = test::get_file_content(test_name, file_renamed.as_str());
    assert_eq!(actual_content, file_content.to_string());
}

#[test]
fn move_files_in_recursive_mode_rename_subdirectory() {
    // given
    let test_name = "move_files_in_recursive_mode_rename_subdirectory";

    let subdirectory_1 = "subdirectory_1";
    let subdirectory_2 = "subdirectory_2";

    let files = vec![];

    let from = format!("./{subdirectory_1}");
    let to = format!("./{subdirectory_2}");
    let renames = vec![test::Rename {
        from: from.as_str(),
        to: to.as_str(),
    }];

    test::init_with_subdirectories(test_name, vec![subdirectory_1.to_string()], files);
    let args = vec!["-rp", "."];

    // when
    test::bulkmv(test_name, args, renames);

    // then
    let subdirectories = test::get_dir_items(test_name);
    assert_eq!(subdirectories, vec![subdirectory_2]);
}
