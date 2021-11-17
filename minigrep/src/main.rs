use std::env;
use std::io;
use std::fs;


struct Config {
    query: String,
    filename: String,
}


fn parse_args() -> Config {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 3 {
        panic!("Usage: cargo minigrep <query> <filename>");
    }

    return Config{
        query: arguments[1].clone(),
        filename: arguments[2].clone(),
    }
}


fn get_file_as_str(filename: &str) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    return Ok(contents);
}


fn minigrep<'a>(contents: &'a str, query: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if !line.to_lowercase().contains(&query.to_lowercase()) {
            continue;
        }

        result.push(&line);
    }

    return result;
}


fn main() {
    let config = parse_args();
    let contents = get_file_as_str(&config.filename).expect("Error: while reading from file");
    let result = minigrep(&contents, &config.query);
    if result.len() == 0 {
        println!("No matching rows found!!");
        return;
    }

    for row in result {
        println!("{}", row);
    }
}
