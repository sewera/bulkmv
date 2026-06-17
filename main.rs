use std::{env, fs, process};

fn main() {
    println!("Hello World");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <READ_DIR>", get_arg(args, 0));
        process::exit(1);
    }
    let dir = get_dir_items(get_arg(args, 1));

    println!("{:?}", dir);
}

fn get_arg(args: Vec<String>, index: usize) -> String {
    args.iter().nth(index).unwrap().to_string()
}

fn get_dir_items(path: String) -> Vec<fs::DirEntry> {
    let dir = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("error: {}", err.to_string());
        process::exit(2);
    });

    let dir_contents = dir.map(|res| res.unwrap_or_else(|err| {
        eprintln!("error: {}", err.to_string());
        process::exit(2);
    })).collect::<Vec<_>>();

    let mut sorted = dir_contents;
    sorted.sort_by(|a, b| a.path().cmp(&b.path()));
    sorted
}
