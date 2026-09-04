use std::io::Write;
use std::{env, fs, io, path, process};

macro_rules! debug {
    ($($arg:tt)*) => {
        cfg_select! {
            debug_assertions => {
                eprint!("DEBUG: ");
                eprintln!($($arg)*);
            },
            _ => {}
        }
    };
}

const TMP_FILE_PATH: &'static str = "BULKMV_FILE";

fn main() {
    let config = parse_args();
    let current_items = get_dir_items(config.clone());

    debug!("current items: {:?}", current_items);
    debug!("current working directory: {:?}", env::current_dir());
    let tmp_file_path = write_dir_items_to_temp_file(&current_items);
    open_editor(&tmp_file_path);
    let items_to_rename = read_dir_items_from_temp_file();

    if config.verbose {
        print_dir_items_to_rename(&current_items, &items_to_rename);
    }
    rename_files(&current_items, &items_to_rename);
    delete_temp_file(&tmp_file_path);
}

#[derive(Clone)]
struct Config {
    directory: String,
    recursive: bool,
    verbose: bool,
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        err_exit(format!("usage: {} <dir>", get_arg(&args, 0)));
    }
    let path = get_arg(&args, 1);
    if !is_dir(path) {
        err_exit(format!("error: {} is not a directory", path));
    }

    set_working_directory();

    Config {
        directory: path.to_string(),
        recursive: false,
        verbose: false,
    }
}

fn set_working_directory() {
    cfg_select! {
        debug_assertions => {
            let working_directory = env::var("BULKMV_CWD").unwrap_or(String::new());
            if !working_directory.trim().is_empty() {
                debug!("changing working directory to {}", working_directory);
                env::set_current_dir(working_directory).unwrap_or_else(os_err_exit);
            }
        }
        _ => {}
    }
}

fn get_arg(args: &Vec<String>, index: usize) -> &str {
    if index > 1 {
        todo!("more than one arg is not implemented yet")
    }
    args.iter().nth(index).unwrap().as_str()
}

fn open_editor(file_path: &String) {
    debug!("opening editor: {}", file_path);
    let editor_cmd = env::var("EDITOR").unwrap_or(String::new());
    let (cmd, args) = parse_command(editor_cmd);
    let exit_status = process::Command::new(cmd)
        .args(args)
        .arg(file_path)
        .status()
        .unwrap_or_else(os_err_exit);

    if !exit_status.success() {
        err_exit(format!("error: editor exited with {}", exit_status));
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
        err_exit("error: wrong number of items to rename".to_string());
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            fs::rename(current_item, item_to_rename).unwrap_or_else(os_err_exit);
        })
}

fn print_dir_items_to_rename(current_items: &Vec<String>, items_to_rename: &Vec<String>) {
    if current_items.len() != items_to_rename.len() {
        err_exit("error: wrong number of items to rename".to_string());
    }

    current_items
        .iter()
        .zip(items_to_rename.iter())
        .for_each(|(current_item, item_to_rename)| {
            println!("{} -> {}", current_item, item_to_rename);
        })
}

fn delete_temp_file(tmp_file_path: &String) {
    fs::remove_file(tmp_file_path).unwrap_or_else(os_err_exit);
    debug!("deleted: {}", tmp_file_path);
}

fn write_dir_items_to_temp_file(dir_items: &Vec<String>) -> String {
    let file = fs::File::create(TMP_FILE_PATH).unwrap_or_else(os_err_exit);
    let mut buffer = io::BufWriter::new(&file);
    buffer
        .write_all(dir_items.join("\n").as_bytes())
        .unwrap_or_else(os_err_exit);
    file.sync_all().unwrap_or_else(os_err_exit);

    debug!("wrote to {}", TMP_FILE_PATH);
    let path = path::Path::new(TMP_FILE_PATH)
        .canonicalize()
        .unwrap_or_else(|err| {
            debug!("failed to canonicalize: {}", err);
            os_err_exit(err)
        });
    path.to_string_lossy().to_string()
}

fn read_dir_items_from_temp_file() -> Vec<String> {
    let file_content = fs::read_to_string(TMP_FILE_PATH).unwrap_or_else(os_err_exit);
    let lines: Vec<&str> = file_content.split('\n').collect();
    lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn is_dir(path: &str) -> bool {
    fs::metadata(path).unwrap_or_else(os_err_exit).is_dir()
}

fn get_dir_items(config: Config) -> Vec<String> {
    let path = config.directory;
    if config.recursive {
        todo!("recursive is not implemented yet")
    }

    let dir = fs::read_dir(path).unwrap_or_else(os_err_exit);

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(os_err_exit)).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    map_dir_entries_to_strings(&dir_contents)
}

fn map_dir_entries_to_strings(dir_items: &Vec<fs::DirEntry>) -> Vec<String> {
    dir_items
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_string_lossy().into_owned())
        .collect()
}

fn err_exit(error: String) {
    eprintln!("{}", error);
    force_delete_temp_file();
    process::exit(1);
}

fn os_err_exit<T>(err: io::Error) -> T {
    eprintln!("error: {}", err.to_string());
    force_delete_temp_file();
    process::exit(2);
}

fn force_delete_temp_file() {
    fs::remove_file(TMP_FILE_PATH).unwrap_or(());
    debug!("force deleted {}", TMP_FILE_PATH);
}
