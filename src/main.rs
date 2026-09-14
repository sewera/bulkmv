use flag::Config;
use fs::{DirEntry, File};
use std::collections::HashSet;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

mod debug;
mod editor;
mod exit;
mod flag;

const TMP_FILE_PATH: &'static str = "BULKMV_FILE";

fn main() {
    let config = flag::parse();
    let current_items = list_directory(config.clone());

    debug!("current items: {:?}", current_items);
    debug!("current working directory: {:?}", std::env::current_dir());
    let tmp_file_path = write_dir_items_to_temp_file(&current_items);
    editor::open(&tmp_file_path);
    let target_file_names = read_dir_items_from_temp_file();

    let parent_dirs_for_cleanup = if config.create_parent_dirs {
        create_parent_dirs_and_get_parent_dirs_for_cleanup(&current_items, &target_file_names)
    } else {
        vec![]
    };

    let renames = get_renames(&current_items, &target_file_names);
    if config.verbose {
        print_dir_items_to_rename(&renames);
    }
    rename_files(&renames);

    parent_dirs_for_cleanup.iter().for_each(|parent| {
        fs::remove_dir(parent).unwrap_or_else(|_| {
            eprintln!("warning: cannot clean up directory `{}`; not removing it recursively to avoid data loss", parent.display())
        })
    });

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

fn list_directory(config: Config) -> Vec<String> {
    let path = config.directory;

    let dir_contents = if config.recursive {
        list_dir_recursive(path.into())
    } else {
        list_single_dir(path.into())
    };

    map_dir_entries_to_strings(&dir_contents, config.use_paths)
}

fn list_dir_recursive(path: PathBuf) -> Vec<DirEntry> {
    let dir = path.read_dir().unwrap_or_else(exit::os_err);
    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(exit::os_err)).collect();
    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
        .into_iter()
        .flat_map(|file| {
            let file_type = file.file_type().unwrap_or_else(exit::os_err);
            if file_type.is_dir() {
                list_dir_recursive(file.path())
            } else {
                vec![file]
            }
        })
        .collect()
}

fn list_single_dir(path: PathBuf) -> Vec<DirEntry> {
    let dir = path.read_dir().unwrap_or_else(exit::os_err);

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(exit::os_err)).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
}

fn map_dir_entries_to_strings(dir_items: &Vec<DirEntry>, use_paths: bool) -> Vec<String> {
    if use_paths {
        dir_items
            .iter()
            .map(|dir_entry| dir_entry.path().to_string_lossy().to_string())
            .collect()
    } else {
        dir_items
            .iter()
            .map(|dir_entry| dir_entry.file_name().to_string_lossy().to_string())
            .collect()
    }
}

fn write_dir_items_to_temp_file(dir_items: &Vec<String>) -> String {
    let file = File::create(TMP_FILE_PATH).unwrap_or_else(exit::os_err);
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

fn create_parent_dirs_and_get_parent_dirs_for_cleanup(
    current_file_names: &Vec<String>,
    target_file_names: &Vec<String>,
) -> Vec<PathBuf> {
    let target_parents = unique_parents(target_file_names);

    target_parents
        .iter()
        .filter(|parent| !parent.try_exists().unwrap_or_else(exit::os_err))
        .for_each(|parent| fs::create_dir_all(parent).unwrap_or_else(exit::os_err));

    let current_parents = unique_parents(current_file_names);

    current_parents
        .difference(&target_parents)
        .cloned()
        .collect()
}

fn unique_parents(paths: &Vec<String>) -> HashSet<PathBuf> {
    let mut parents: HashSet<PathBuf> = HashSet::new();
    paths
        .into_iter()
        .map(|target| {
            let path = Path::new(target.as_str());
            path.parent()
        })
        .flat_map(|parent| parent.into_iter())
        .filter(|parent| *parent != Path::new("") && *parent != Path::new(".")) // TODO: check if this filter can be simplified
        .for_each(|parent| {
            parents.insert(parent.to_path_buf());
        });
    parents
}

fn temporary_name(name: &String) -> String {
    format!("{name}_TEMP")
}

fn delete_temp_file(tmp_file_path: &String) {
    fs::remove_file(tmp_file_path).unwrap_or_else(exit::os_err);
    debug!("deleted: {}", tmp_file_path);
}
