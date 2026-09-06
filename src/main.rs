use flag::Config;
use std::collections::HashSet;
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
    let target_file_names = read_dir_items_from_temp_file();
    let renames = get_renames(&current_items, &target_file_names);
    if config.verbose {
        print_dir_items_to_rename(&renames);
    }
    rename_files(&renames);
    delete_temp_file(&tmp_file_path);
}

fn get_renames(current_file_names: &Vec<String>, target_file_names: &Vec<String>) -> Vec<Rename> {
    if current_file_names.len() != target_file_names.len() {
        exit::err("error: wrong number of items to rename".to_string())
    }
    verify_target_file_names_unique(target_file_names);

    let renames: Vec<_> = current_file_names
        .iter()
        .zip(target_file_names.iter())
        .filter(|(current, target)| current != target)
        .map(|(current, target)| {
            current_file_names
                .iter()
                .find(|&item| item.eq(target))
                .map(|_| Rename {
                    current: current.into(),
                    target: target.into(),
                    collision: true,
                })
                .unwrap_or_else(|| Rename {
                    current: current.into(),
                    target: target.into(),
                    collision: false,
                })
        })
        .collect();

    renames
}

fn verify_target_file_names_unique(target_file_names: &Vec<String>) {
    let mut set = HashSet::new();
    let mut non_unique = HashSet::new();
    target_file_names.iter().for_each(|item| {
        if !set.insert(item.clone()) {
            non_unique.insert(item.clone());
        }
    });

    if !non_unique.is_empty() {
        let non_unique_formatted = non_unique
            .iter()
            .fold(String::new(), |acc, item| format!("{} {}", acc, item));
        exit::err(format!(
            "error: duplicate target names:{}",
            non_unique_formatted
        ))
    }
}

#[derive(Clone)]
struct Rename {
    current: String,
    target: String,
    collision: bool,
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

fn print_dir_items_to_rename(renames: &Vec<Rename>) {
    renames
        .iter()
        .for_each(|rename| println!("{} -> {}", rename.current, rename.target))
}

fn rename_files(renames: &Vec<Rename>) {
    renames
        .iter()
        .filter(|rename| rename.collision)
        .for_each(|rename| {
            fs::rename(&rename.current, temporary_name(&rename.current))
                .unwrap_or_else(exit::os_err)
        });
    renames
        .iter()
        .filter(|rename| !rename.collision)
        .for_each(|rename| {
            fs::rename(&rename.current, &rename.target).unwrap_or_else(exit::os_err)
        });
    renames
        .iter()
        .filter(|rename| rename.collision)
        .for_each(|rename| {
            fs::rename(temporary_name(&rename.current), &rename.target).unwrap_or_else(exit::os_err)
        });
}

fn temporary_name(name: &String) -> String {
    format!("{name}_TEMP")
}

fn delete_temp_file(tmp_file_path: &String) {
    fs::remove_file(tmp_file_path).unwrap_or_else(exit::os_err);
    debug!("deleted: {}", tmp_file_path);
}
