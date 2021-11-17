use std::env;
use std::error;
use std::fs;
use std::process;


struct Config {
    query: String,
    filename: String,
}


fn parse_args() -> Config {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 3 {
        println!("Usage: cargo minigrep <query> <filename>");
        process::exit(-1);
    }

    return Config{
        query: arguments[1].clone(),
        filename: arguments[2].clone(),
    }
}


fn get_file_as_str(filename: &str) -> Result<String, Box<dyn error::Error>> {
    let contents = fs::read_to_string(filename)?;
    return Ok(contents);
}


fn minigrep<'a>(contents: &'a str, query: & str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if !line.to_lowercase().contains(&query.to_lowercase()) {
            continue;
        }

        result.push(line);
    }

    return result;
}


fn main() {
    let config = parse_args();
    let contents = get_file_as_str(&config.filename).unwrap_or_else(|err| {
        eprintln!("Error: {} while reading from file {}", err, config.filename);
        process::exit(-1);
    });
    let result = minigrep(&contents, &config.query);
    if result.len() == 0 {
        println!("No matching rows found!!");
        return;
    }

    for row in result {
        println!("{}", row);
    }
}
