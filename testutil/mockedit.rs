use std::{env, fmt, fs, process};

fn main() {
    let config = parse_args();
    edit_temp_file(config);
    eprintln!("mockedit: ok");
    process::exit(0);
}

fn edit_temp_file(config: Config) {
    let temp_file_content = read_temp_file(&config.temp_file);
    let new_temp_file_content = change_filenames(temp_file_content, &config.renames);
    write_temp_file(&config.temp_file, new_temp_file_content);
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

fn parse_args() -> Config {
    let args = env::args().collect();
    validate_args(&args);
    let temp_file = args.last().unwrap().clone();
    let rename_pairs = args[1..args.len() - 1].to_vec();

    let renames: Vec<_> = rename_pairs
        .chunks_exact(2)
        .map(|pair| Rename {
            current: pair[0].clone(),
            target: pair[1].clone(),
        })
        .collect();

    let config = Config { temp_file, renames };
    eprintln!("mockedit: config: {}", config);
    config
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

fn read_temp_file(temp_file: &String) -> String {
    let temp_file_content = fs::read_to_string(temp_file).unwrap_or_else(|err| {
        eprintln!(
            "mockedit: current working directory: {}",
            env::current_dir().unwrap().to_string_lossy()
        );
        eprintln!("mockedit: read_temp_file: {}: {}", temp_file, err);
        process::exit(err.raw_os_error().unwrap_or(1));
    });
    eprintln!(
        "mockedit: old temp file content: |\n{}\nEOF",
        temp_file_content
    );
    temp_file_content
}

fn change_filenames(temp_file_content: String, renames: &Vec<Rename>) -> String {
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

fn write_temp_file(temp_file: &String, new_temp_file_content: String) {
    eprintln!(
        "mockedit: new temp file content: |\n{}\nEOF",
        new_temp_file_content
    );
    fs::write(temp_file, new_temp_file_content).unwrap();
}
