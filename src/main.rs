use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn split_on_word(line: &str, word: &str) -> Option<(String, String, String)> {
    let pattern = format!(r"(?i)\b{}\b", regex::escape(word));
    let regex = Regex::new(&pattern).unwrap();

    if let Some(index) = line.split_whitespace().position(|w| regex.is_match(w)) {
        let first_part = line
            .split_whitespace()
            .take(index)
            .collect::<Vec<&str>>()
            .join(" ");
        let second_part = line.split_whitespace().nth(index).unwrap().to_string();
        let third_part = line
            .split_whitespace()
            .skip(index + 1)
            .collect::<Vec<&str>>()
            .join(" ");
        Some((first_part, second_part, third_part))
    } else {
        None
    }
}

fn main() -> Result<(), std::io::Error> {
    let cli_matches = App::new("A regex clone written in Rust")
        .arg(
            Arg::new("first_match_only")
                .short('e')
                .long("first-match")
                .help("Finds the First Match"),
        )
        .arg(
            Arg::new("whole-file")
                .short('s')
                .long("whole-file")
                .help("Returns the whole file with matches highlighted"),
        )
        .arg(Arg::new("pattern").required(true).help("Pattern to match"))
        .arg(Arg::new("file").help("File to search in"))
        .get_matches();

    let whole_match = cli_matches.is_present("whole-file");
    let first_match = cli_matches.is_present("first_match_only");
    let pattern = cli_matches.value_of("pattern").unwrap();
    let reader: BufReader<Box<dyn Read>>;
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    
    if let Some(file) = cli_matches.value_of("file") {
        // let file = match File::open(cli_matches.value_of("file").unwrap()) {
        //     Err(why) => {
        //         eprintln!("Error opening file {}", why);
        //         std::process::exit(1)
        //     }
        //     Ok(file) => file,
        // };
        reader = BufReader::new(Box::new(File::open(file).expect("Error")));
    } else {
        reader = BufReader::new(Box::new(io::stdin()));
    }


    if whole_match {
        let regex_string = format!(r"(?i)\b{}\b", pattern);
        let re = Regex::new(&regex_string).unwrap();

        let mut stdout_handle = stdout.lock();
        let mut color_spec = ColorSpec::new();
        color_spec.set_bold(true);
        color_spec.set_fg(Some(Color::Red));

        for line in reader.lines() {
            let line = line?;
            let mut start = 0;
            if let Some(matched) = re.find(&line) {
                let prefix = &line[..matched.start()];
                let matched_str = &line[matched.start()..matched.end()];
                let suffix = &line[matched.end()..];

                stdout_handle.write_all(prefix.as_bytes())?;
                stdout_handle.set_color(&color_spec)?;
                stdout_handle.write_all(matched_str.as_bytes())?;
                stdout_handle.reset()?;
                stdout_handle.write_all(suffix.as_bytes())?;

                start = matched.end();
            }

            stdout_handle.write_all(line[start..].as_bytes())?;
            stdout_handle.write_all(b"\n")?;
            stdout_handle.flush()?;
        }
    } else {
        let mut n = 1;
        let regex_string = format!(r"(?i)(.*)\b{}\b.*$", pattern);
        // let regex_strings = format!(r"\b{}\b", regex::escape(pattern));
        let re = Regex::new(&regex_string).unwrap();
        // let rex = Regex::new(&regex_strings).unwrap();
        // let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        for line in reader.lines() {
            let line = line?;
            if re.is_match(&line) {
                // let words = line.split(' ').collect::<Vec<&str>>();
                if let Some((first_part, second_part, third_part)) = split_on_word(&line, pattern) {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                    write!(&mut stdout, "Match {}: ", &n)?;
                    stdout.reset()?;

                    write!(&mut stdout, "{} ", first_part)?;
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    write!(&mut stdout, "{} ", second_part)?;
                    stdout.reset()?;
                    writeln!(&mut stdout, "{}", third_part)?;
                    println!();
                } else {
                    writeln!(&mut stdout, "{} ", line)?;
                }

                if first_match {
                    break;
                }
                n += 1;
            }
        }
    }
    Ok(())
}
