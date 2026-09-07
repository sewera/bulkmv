use crate::debug;
use crate::exit;
use std::{env, fs, path};

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) directory: String,
    pub(crate) recursive: bool,
    pub(crate) verbose: bool,
    pub(crate) use_paths: bool,
}

#[derive(Eq, PartialEq)]
pub(crate) enum Flag {
    Unknown(String),
    Separator,
    Recursive,
    Verbose,
    Help,
}

pub(crate) const FLAG_PREFIX: &'static str = "-";
pub(crate) const FLAG_SEPARATOR: &'static str = "--";
pub(crate) const FLAG_RECURSIVE_SHORT: char = 'r';
pub(crate) const FLAG_RECURSIVE_LONG: &'static str = "--recursive";
pub(crate) const FLAG_VERBOSE_SHORT: char = 'v';
pub(crate) const FLAG_VERBOSE_LONG: &'static str = "--verbose";
pub(crate) const FLAG_HELP_SHORT: char = 'h';
pub(crate) const FLAG_HELP_LONG: &'static str = "--help";

pub(crate) fn parse() -> Config {
    let all_args: Vec<String> = env::args().collect();
    let args: Vec<String> = all_args.iter().skip(1).map(move |x| x.into()).collect();
    let flags = parse_flags(&args);
    if flags.contains(&Flag::Help) {
        exit::print_usage()
    }

    set_working_directory();

    let current_working_dir = env::current_dir()
        .unwrap_or_else(exit::os_err)
        .to_string_lossy()
        .to_string();
    let directory = args
        .iter()
        .position(|arg| arg.eq(FLAG_SEPARATOR))
        .and_then(|separator_index| args.get(separator_index + 1))
        .or_else(|| args.iter().find(|arg| !arg.starts_with(FLAG_PREFIX)))
        .map(|it| it.clone())
        .unwrap_or(current_working_dir.clone());

    if !is_dir(&directory) {
        exit::err(format!("error: {} is not a directory", directory))
    }

    let same_path_as_cwd = path::Path::new(&directory.as_str())
        .canonicalize()
        .map(|full_path| {
            path::Path::new(current_working_dir.as_str())
                .canonicalize()
                .map(|full_cwd| full_path == full_cwd)
                .unwrap_or(false)
        })
        .ok()
        .unwrap_or(false);

    let recursive = flags.contains(&Flag::Recursive);
    let verbose = flags.contains(&Flag::Verbose);
    let use_paths = recursive || !same_path_as_cwd;

    Config {
        directory,
        recursive,
        verbose,
        use_paths,
    }
}

fn parse_flags(args: &Vec<String>) -> Vec<Flag> {
    let flags: Vec<_> = args
        .iter()
        .filter(|arg| arg.starts_with(FLAG_PREFIX))
        .flat_map(|arg| match arg.as_str() {
            FLAG_VERBOSE_LONG => vec![Flag::Verbose],
            FLAG_RECURSIVE_LONG => vec![Flag::Recursive],
            FLAG_HELP_LONG => vec![Flag::Help],
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
            FLAG_HELP_SHORT => Flag::Help,
            s => Flag::Unknown(s.to_string()),
        })
        .collect()
}

fn is_dir(path: &String) -> bool {
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
