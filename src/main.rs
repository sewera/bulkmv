use flag::Config;
use std::env::current_dir;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

mod debug;
mod editor;
mod exit;
mod flag;

const TMP_FILE_PATH: &'static str = "BULKMV_FILE";

fn main() {
    let config = flag::parse();
    let current_items = get_dir_items(config.clone());

    debug!("current items: {:?}", current_items);
    debug!("current working directory: {:?}", current_dir());
    let tmp_file_path = write_dir_items_to_temp_file(&current_items);
    editor::open(&tmp_file_path);
    let items_to_rename = read_dir_items_from_temp_file();

    if config.verbose {
        print_dir_items_to_rename(&current_items, &items_to_rename);
    }
    rename_files(&current_items, &items_to_rename);
    delete_temp_file(&tmp_file_path);
}

fn get_dir_items(config: Config) -> Vec<String> {
    let path = config.directory;
    if config.recursive {
        todo!("recursive is not implemented yet")
    }

    let dir = fs::read_dir(path).unwrap_or_else(exit::os_err);

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(exit::os_err)).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    map_dir_entries_to_strings(&dir_contents)
}

fn map_dir_entries_to_strings(dir_items: &Vec<fs::DirEntry>) -> Vec<String> {
    dir_items
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_string_lossy().into_owned())
        .collect()
}

fn write_dir_items_to_temp_file(dir_items: &Vec<String>) -> String {
    let file = fs::File::create(TMP_FILE_PATH).unwrap_or_else(exit::os_err);
    let mut buffer = BufWriter::new(&file);
    buffer
        .write_all(dir_items.join("\n").as_bytes())
        .unwrap_or_else(exit::os_err);
    file.sync_all().unwrap_or_else(exit::os_err);

    debug!("wrote to {}", TMP_FILE_PATH);
    let path = Path::new(TMP_FILE_PATH)
        .canonicalize()
        .unwrap_or_else(|err| {
            debug!("failed to canonicalize: {}", err);
            exit::os_err(err)
        });
    path.to_string_lossy().to_string()
}

fn read_dir_items_from_temp_file() -> Vec<String> {
    let file_content = fs::read_to_string(TMP_FILE_PATH).unwrap_or_else(exit::os_err);
    let lines: Vec<&str> = file_content.split('\n').collect();
    lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn print_dir_items_to_rename(current_items: &Vec<String>, items_to_rename: &Vec<String>) {
    if current_items.len() != items_to_rename.len() {
        exit::err("error: wrong number of items to rename".to_string())
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            println!("{} -> {}", current_item, item_to_rename);
        })
}

fn rename_files(current_items: &Vec<String>, items_to_rename: &Vec<String>) {
    if current_items.len() != items_to_rename.len() {
        exit::err("error: wrong number of items to rename".to_string())
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            fs::rename(current_item, item_to_rename).unwrap_or_else(exit::os_err);
        })
}

fn delete_temp_file(tmp_file_path: &String) {
    fs::remove_file(tmp_file_path).unwrap_or_else(exit::os_err);
    debug!("deleted: {}", tmp_file_path);
}
