use std::{env, fs, io, process};

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

    let dir = get_dir_items(path);
    print_dir_items(&dir);
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

fn get_dir_items(path: &str) -> Vec<fs::DirEntry> {
    let dir = fs::read_dir(path).unwrap_or_else(os_err);

    let mut dir_contents: Vec<_> = dir.map(|res| res.unwrap_or_else(os_err)).collect();

    dir_contents.sort_by(|a, b| a.path().cmp(&b.path()));
    dir_contents
}

fn print_dir_items(dir: &Vec<fs::DirEntry>) {
    dir.iter()
        .for_each(|entry| println!("{}", entry.path().to_str().unwrap()));

    // TODO: Accept a file or a writer and instead of printing lines, write to that file
}

fn os_err<T>(err: io::Error) -> T {
    eprintln!("error: {}", err.to_string());
    process::exit(2);
}

// TODO: Create a test for printing directory items
