use crate::{debug, TMP_FILE_PATH};
use std::{env, fs, io, process};

pub(crate) fn err_usage<T>() -> T {
    let executable: String = env::args()
        .next()
        .unwrap_or_else(|| err("error: could not get executable name".into()));
    err(format!("usage: {} [-v|--verbose] <dir>", executable))
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
