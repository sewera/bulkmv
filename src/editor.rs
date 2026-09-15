use crate::{debug, exit};
use std::{env, process};

pub fn open(file_path: &String) {
    debug!("opening editor: {}", file_path);
    let editor_cmd = editor();
    let (cmd, args) = parse_command(editor_cmd);
    let exit_status = process::Command::new(cmd)
        .args(args)
        .arg(file_path)
        .status()
        .unwrap_or_else(exit::os_err);

    if !exit_status.success() {
        exit::err(format!("error: editor exited with {}", exit_status))
    }
}

fn editor() -> String {
    const ENV_VISUAL: &'static str = "VISUAL";
    const ENV_EDITOR: &'static str = "EDITOR";
    env::var(ENV_VISUAL)
        .ok()
        .filter(|s| !s.is_empty())
        .or(env::var(ENV_EDITOR).ok())
        .filter(|s| !s.is_empty())
        .unwrap_or(DEFAULT_EDITOR.into())
}

#[cfg(unix)]
const DEFAULT_EDITOR: &'static str = "vi";

#[cfg(windows)]
const DEFAULT_EDITOR: &'static str = "notepad.exe";

fn parse_command(cmdline: String) -> (String, Vec<String>) {
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
