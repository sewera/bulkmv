use crate::debug;
use crate::exit;
use std::{env, fs};

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) directory: String,
    pub(crate) recursive: bool,
    pub(crate) verbose: bool,
}

#[derive(Eq, PartialEq)]
pub(crate) enum Flag {
    Unknown(String),
    Separator,
    Recursive,
    Verbose,
}

pub(crate) const FLAG_PREFIX: &'static str = "-";
pub(crate) const FLAG_SEPARATOR: &'static str = "--";
pub(crate) const FLAG_RECURSIVE_SHORT: char = 'r';
pub(crate) const FLAG_RECURSIVE_LONG: &'static str = "--recursive";
pub(crate) const FLAG_VERBOSE_SHORT: char = 'v';
pub(crate) const FLAG_VERBOSE_LONG: &'static str = "--verbose";

pub(crate) fn parse() -> Config {
    let all_args: Vec<String> = env::args().collect();
    if all_args.len() < 2 {
        exit::err_usage()
    }
    let args: Vec<String> = all_args.iter().skip(1).map(move |x| x.into()).collect();

    let flags = parse_flags(&args);

    let flag_separator = args.iter().position(|arg| arg.eq(FLAG_SEPARATOR));
    let path = flag_separator
        .and_then(|separator_index| args.get(separator_index + 1))
        .or_else(|| args.iter().find(|arg| !arg.starts_with(FLAG_PREFIX)))
        .unwrap_or_else(exit::err_usage);

    if !is_dir(path) {
        exit::err(format!("error: {} is not a directory", path))
    }

    set_working_directory();

    Config {
        directory: path.into(),
        recursive: flags.contains(&Flag::Recursive),
        verbose: flags.contains(&Flag::Verbose),
    }
}

fn parse_flags(args: &Vec<String>) -> Vec<Flag> {
    let flags: Vec<_> = args
        .iter()
        .filter(|arg| arg.starts_with(FLAG_PREFIX))
        .flat_map(|arg| match arg.as_str() {
            FLAG_VERBOSE_LONG => vec![Flag::Verbose],
            FLAG_RECURSIVE_LONG => vec![Flag::Recursive],
            FLAG_SEPARATOR => vec![Flag::Separator],
            s => parse_short_flags(s),
        })
        .collect();

    let unknown_flags: Vec<_> = flags
        .iter()
        .filter(|flag| match flag {
            Flag::Unknown(_) => true,
            _ => false,
        })
        .map(|unknown_flag| match unknown_flag {
            Flag::Unknown(s) => s.into(),
            _ => String::new(),
        })
        .collect();

    if !unknown_flags.is_empty() {
        eprintln!("error: unknown flag: {}", unknown_flags.join(" "));
        exit::err_usage()
    }
    flags
}

fn parse_short_flags(arg: &str) -> Vec<Flag> {
    let short_flags = arg.strip_prefix(FLAG_PREFIX).unwrap_or("");
    short_flags
        .chars()
        .map(|short_flag| match short_flag {
            FLAG_VERBOSE_SHORT => Flag::Verbose,
            FLAG_RECURSIVE_SHORT => Flag::Recursive,
            s => Flag::Unknown(s.to_string()),
        })
        .collect()
}

fn is_dir(path: &str) -> bool {
    fs::metadata(path).unwrap_or_else(exit::os_err).is_dir()
}

fn set_working_directory() {
    cfg_select! {
        debug_assertions => {
            let working_directory = env::var("BULKMV_CWD").unwrap_or(String::new());
            if !working_directory.trim().is_empty() {
                debug!("changing working directory to {}", working_directory);
                env::set_current_dir(working_directory).unwrap_or_else(exit::os_err);
            }
        }
        _ => {}
    }
}
