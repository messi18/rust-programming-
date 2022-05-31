use std::{fs, process};
use std::error::Error;

fn simple_grep(args:&[String])  {
    let config = Config::new(args).unwrap_or_else(|e| {
        println!("error happened:{}", e);
        process::exit(1);
    });
    println!("fileName:{}", config.fileName);
    println!("text:{}", config.text);
    let contents = run(config).unwrap_or_else(|e| {
        eprintln!("Error happened:{}", e);
        process::exit(1);
    });

}

fn run(config: Config) -> Result<String,Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.fileName)?;
    Ok(contents)
}
struct Config {
    fileName: String,
    text: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 3 {
            return Err("too much arguments than 3.")
        }
        let fileName = args[1].clone();
        let text = args[2].clone();
        return Ok(Config { fileName, text });
    }
}