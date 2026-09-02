use std::{env, fmt, fs, io, process};

fn main() {
    let args: Vec<_> = env::args().collect();
    validate_args(&args);
    let config = parse_args(&args);
    println!("mockedit: config: {}", config);

    let temp_file_content = read_temp_file(&config.temp_file);
    println!("mockedit: old temp file content: |\n{}", &temp_file_content);

    let new_temp_file_content = make_filename_changes(temp_file_content, &config.renames);
    println!(
        "mockedit: new temp file content: |\n{}",
        &new_temp_file_content
    );

    write_temp_file(&config.temp_file, new_temp_file_content);

    println!("mockedit: ok");
    process::exit(0);
}

fn write_temp_file(temp_file: &String, new_temp_file_content: String) {
    fs::write(temp_file, new_temp_file_content).unwrap_or_else(os_err);
}

fn make_filename_changes(temp_file_content: String, renames: &Vec<Rename>) -> String {
    let mut available_renames = renames.clone();
    temp_file_content
        .lines()
        .map(|line| map_line(line, &mut available_renames))
        .collect::<Vec<String>>()
        .join("\n")
}

fn map_line(line: &str, available_renames: &mut Vec<Rename>) -> String {
    let found = available_renames.iter().position(|r| r.current == line);
    if found.is_none() {
        return line.to_string();
    }
    let found = found.unwrap();
    let renamed = available_renames[found].target.clone();
    available_renames.remove(found);
    renamed
}

fn validate_args(args: &Vec<String>) {
    if args.len() < 2 || args.len() % 2 != 0 {
        eprintln!("mockedit: invalid argument count: {}", args.len());
        eprintln!(
            "mockedit: usage: {} [(<filename> <filename_renamed> )*] <temp_file>",
            args.first().unwrap()
        );
        process::exit(1);
    }
}

fn parse_args(args: &Vec<String>) -> Config {
    let temp_file = args.last().unwrap().clone();
    let rename_pairs = args[1..args.len() - 1].to_vec();

    let renames: Vec<_> = rename_pairs
        .windows(2)
        .map(|pair| Rename {
            current: pair[0].clone(),
            target: pair[1].clone(),
        })
        .collect();

    Config { temp_file, renames }
}

#[derive(Clone)]
struct Config {
    temp_file: String,
    renames: Vec<Rename>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "temp_file: {}; ", self.temp_file)?;
        write!(f, "renames: [")?;
        self.renames
            .iter()
            .for_each(|r| write!(f, "{} -> {}, ", r.current, r.target).unwrap());
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone)]
struct Rename {
    current: String,
    target: String,
}

fn read_temp_file(temp_file: &String) -> String {
    fs::read_to_string(temp_file).unwrap_or_else(os_err)
}

fn os_err<T>(err: io::Error) -> T {
    eprintln!("error: {}", err.to_string());
    process::exit(2);
}
