# esgrep

A command-line tool inspired by the classic grep command, providing pattern-matching capabilities within files. esgrep is built using the Rust programming language. If no file is specified for the input, esgrep reads from standard input (STDIN). esgrep uses regular expressions for pattern matching. Consult a regular expression reference for the syntax. 


## Features
- **Pattern Matching:** Search for specific regex patterns within your files.
- **First Match Mode:** Quickly locate the first occurrence of a pattern within a file.
- **Whole File Highlighting:** Display entire files with matching parts prominently highlighted in color.
- **Customization:** Adjust output formatting and color choices (via termcolor).


## Installation
To install esgrep, you will need to have the Rust programming language installed on your system. If you do not have Rust installed, you can do so by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install

1. Clone the repository: `git clone https://github.com/EphraimShikanga/esgrep.git`
2. Build the project: `cargo build --release`
3. Run the executable: `./target/release/esgrep --help`
4. (Optional) Add the executable to your PATH for global access.
5. (Optional) Run the tests: `cargo test`
6. (Optional) Generate the documentation: `cargo doc --open`


## Usage
esgrep is designed to be used in a similar manner to the classic grep command. The basic usage is as follows:

```
esgrep [FLAGS] [OPTIONS] <PATTERN> <FILE>...
```

### Flags
- `-e, --first-match`: Display only the first match within each file.
- `-s, --whole-file`: Display the entire file with matching parts highlighted.
- `-c, --color`: Display the matching parts in the specified color (red, green, yellow, blue, magenta, cyan, white).
- `-h, --help`: Display the help message.
- `-V, --version`: Display the version information.


## Examples
1. Search for the word "error" in a file:
```
esgrep error file.txt
```

2. Search for the word "warning" in multiple files:
```
esgrep warning file1.txt file2.txt file3.txt
```

3. Search for the word "success" in a file and display the first match:
```
esgrep -e success file.txt
```

4. Search for the word "info" in a file and display the entire file with matching parts highlighted:
```
esgrep -s info file.txt
```

5. Search for the word "debug" in a file and display the entire file with matching parts highlighted in red:
```
esgrep -s debug -c red file.txt
```

6. Find lines containing email addresses within a text file, highlighting matches:
```
esgrep -s -c yellow "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" file.txt
```

## Notes
- If no file is specified for the input, esgrep reads from standard input (STDIN).
- esgrep is a work in progress and may contain bugs or incomplete features.
- Contributions are welcome! Feel free to submit a pull request or open an issue.


    
    
    
    
