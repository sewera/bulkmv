use std::{env, fs, io, process};

const TMP_FILE_PATH: &'static str = "BULKMV_FILE";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <dir>", get_arg(&args, 0));
        process::exit(1);
    }
    let path = get_arg(&args, 1);
    if !is_dir(path) {
        eprintln!("error: {} is not a directory", path);
        process::exit(1);
    }

    let current_items = get_dir_items(path);
    println!("Write to file");
    write_dir_items_to_temp_file(&current_items);

    open_editor(TMP_FILE_PATH);

    println!("Read from file");
    let items_to_rename = read_dir_items_from_temp_file();

    print_dir_items_to_rename(&current_items, &items_to_rename);

    rename_files(&current_items, &items_to_rename);

    delete_temp_file();
}

fn open_editor(file_path: &str) {
    let editor_cmd = env::var("EDITOR").unwrap_or(String::new());
    let (cmd, args) = parse_command(editor_cmd);
    let exit_status = process::Command::new(cmd)
        .args(args)
        .arg(file_path)
        .status()
        .unwrap_or_else(os_err);

    if !exit_status.success() {
        eprintln!("error: editor exited with {}", exit_status);
        process::exit(1);
    }
}

fn parse_command(cmdline: String) -> (String, Vec<String>) {
    const DEFAULT_EDITOR: &'static str = "vi";

    if cmdline.is_empty() {
        return (DEFAULT_EDITOR.to_owned(), Vec::new());
    }

    let elements: Vec<_> = cmdline.split_whitespace().collect();
    if elements.is_empty() {
        return (DEFAULT_EDITOR.to_owned(), Vec::new());
    } else if elements.len() == 1 {
        return (cmdline, Vec::new());
    }

    let (cmd, args) = elements.split_first().unwrap();
    let cmd_str: String = str::parse(cmd).unwrap();
    let args_str_vec: Vec<String> = args.to_vec().iter().map(|s| s.to_string()).collect();

    (cmd_str, args_str_vec)
}

fn rename_files(current_items: &Vec<String>, items_to_rename: &Vec<String>) {
    if current_items.len() != items_to_rename.len() {
        eprintln!("error: wrong number of items to rename");
        process::exit(1);
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            fs::rename(current_item, item_to_rename).unwrap_or_else(os_err);
        })
}

fn print_dir_items_to_rename(current_items: &Vec<String>, items_to_rename: &Vec<String>) {
    if current_items.len() != items_to_rename.len() {
        eprintln!("error: wrong number of items to rename");
        process::exit(1);
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            println!("{} -> {}", current_item, item_to_rename);
        })
}

fn delete_temp_file() {
    fs::remove_file(TMP_FILE_PATH).unwrap_or_else(os_err);
}

fn write_dir_items_to_temp_file(dir_items: &Vec<String>) {
    fs::write(TMP_FILE_PATH, dir_items.join("\n")).unwrap_or_else(os_err);
}

fn read_dir_items_from_temp_file() -> Vec<String> {
    let file_content = fs::read_to_string(TMP_FILE_PATH).unwrap_or_else(os_err);
    let lines: Vec<&str> = file_content.split('\n').collect();
    lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn get_arg(args: &Vec<String>, index: usize) -> &str {
    if index > 1 {
        todo!("more than one arg is not implemented yet")
    }
    args.iter().nth(index).unwrap().as_str()
}

fn is_dir(path: &str) -> bool {
    fs::metadata(path).unwrap_or_else(os_err).is_dir()
}

fn get_dir_items(path: &str) -> Vec<String> {
    let dir = fs::read_dir(path).unwrap_or_else(os_err);

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(os_err)).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    map_dir_entries_to_strings(&dir_contents)
}

fn map_dir_entries_to_strings(dir_items: &Vec<fs::DirEntry>) -> Vec<String> {
    dir_items
        .iter()
        .map(|dir_entry| dir_entry.path().to_string_lossy().into_owned())
        .collect()
}

fn os_err<T>(err: io::Error) -> T {
    eprintln!("error: {}", err.to_string());
    process::exit(2);
}

#[cfg(test)]
mod main_test {
    use super::*;

    const TEST_FILE_NAME: &str = "file1";
    const TEST_FILE_NAME_RENAMED: &str = "file2";
    const TEST_DIR: &str = "testdata/testdir";

    #[test]
    fn rename_one_file() {
        clear_test_dir();
        make_test_dir();
        let current_items = get_dir_items(TEST_DIR);

        write_dir_items_to_temp_file(&current_items);
        let renamed_file_path = format!("{}/{}", TEST_DIR, TEST_FILE_NAME_RENAMED);
        fs::write(TMP_FILE_PATH, &renamed_file_path).unwrap();
        let to_rename = read_dir_items_from_temp_file();

        assert_eq!(to_rename, vec![renamed_file_path.clone()]);
        print_dir_items_to_rename(&current_items, &to_rename);

        rename_files(&current_items, &to_rename);
        delete_temp_file();

        assert_eq!(get_dir_items(TEST_DIR), vec![renamed_file_path.clone()]);
    }

    fn clear_test_dir() {
        fs::remove_dir_all(TEST_DIR).unwrap();
    }

    fn make_test_dir() {
        let test_file_path = format!("{}/{}", TEST_DIR, TEST_FILE_NAME);
        fs::create_dir_all(TEST_DIR).unwrap();
        fs::write(test_file_path, "test data").unwrap();
    }
}
