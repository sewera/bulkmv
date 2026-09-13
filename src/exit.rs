use crate::{debug, TMP_FILE_PATH};
use std::{env, fs, io, process};

pub(crate) fn err_usage<T>() -> T {
    err(usage())
}

pub(crate) fn print_usage() {
    println!("{}", usage());
    process::exit(0);
}

const USAGE: &'static str = "usage: ";
const USAGE_DETAILS: &'static str = " [-vrp] [--] <dir>

  Available flags:
    -v, --verbose  print move commands (e.g., 'file1 -> file2')
    -r, --recursive  recursive mode (work on the whole tree)
    -p, --create-parent-dirs  create parent directories and clean up unused ones
    -h, --help  display this help and exit
  Examples:
    bulkmv  # move files in non-recursive mode in the current directory
    bulkmv -v dir  # move files in non-recursive mode in 'dir' and print move commands
    bulkmv -vrp -- dir  # move files in recursive mode in 'dir',
                        # print move commands, create parent directories,
                        # and clean up unused directories
";

fn usage() -> String {
    let executable: String = env::args()
        .next()
        .unwrap_or_else(|| err("error: could not get executable name".into()));
    format!("{USAGE}{executable}{USAGE_DETAILS}")
}

pub(crate) fn err<T>(error: String) -> T {
    eprintln!("{}", error);
    force_delete_temp_file();
    process::exit(1);
}

pub(crate) fn os_err<T>(err: io::Error) -> T {
    eprintln!("error: {}", err.to_string());
    force_delete_temp_file();
    process::exit(2);
}

fn force_delete_temp_file() {
    fs::remove_file(TMP_FILE_PATH).unwrap_or(());
    debug!("force deleted {}", TMP_FILE_PATH);
}
