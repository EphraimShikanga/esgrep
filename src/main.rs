use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use colored::Colorize;

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
        .arg(Arg::new("file").required(true).help("File to search in"))
        .get_matches();

    let whole_match = cli_matches.is_present("whole-file");
    let first_match = cli_matches.is_present("first_match_only");
    let pattern = cli_matches.value_of("pattern").unwrap();
    let file = match File::open(cli_matches.value_of("file").unwrap()) {
        Err(why) => {
            eprintln!("Error opening file {}", why);
            std::process::exit(1)
        }
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    if whole_match {
        let regex_string = format!(r"\b{}\b", pattern);
        let re = Regex::new(&regex_string).unwrap();
        let stdout = StandardStream::stdout(ColorChoice::Auto);

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
        let regex_string = format!(r"(?i)(\b{}\b)", pattern);
        let re = Regex::new(&regex_string).unwrap();
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        let mut n = 1;

        for line in reader.lines() {
            let line = line?;
            if let Some(matched) = re.find(&line) {
                let mut colored_line = line.clone();
                let matched_word = &line[matched.start()..matched.end()];

                // Replace the matched word with the same word in green
                colored_line.replace_range(
                    matched.start()..matched.end(),
                    &format!("{}", matched_word.green()),
                );
                
                // Print the line with the matched word in green
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                write!(&mut stdout, "Match {}:", n)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", colored_line)?;

                println!();
                if first_match {
                    break;
                }
                n += 1;
            }
        }
    }
    Ok(())
}
