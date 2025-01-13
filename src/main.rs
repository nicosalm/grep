use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(short = 'p', long)]
    pattern: String,

    /// The file/dir to search in
    #[arg(short = 'f', long)]
    path: PathBuf,

    /// Search recursively through directories
    #[arg(short, long)]
    recursive: bool,

    /// Ignore case when matching
    #[arg(short, long)]
    ignore_case: bool,

    /// Print NUM lines of leading context
    #[arg(short = 'B', long = "before-context")]
    before_context: Option<usize>,

    /// Print NUM lines of trailing context
    #[arg(short = 'A', long = "after-context")]
    after_context: Option<usize>,

    /// Print NUM lines of context
    #[arg(short = 'C', long = "context")]
    context: Option<usize>,

    /// Disable colored output
    #[arg(long, default_value_t = false)]
    no_color: bool,

    /// Show line numbers
    #[arg(short = 'n', long, default_value_t = false)]
    line_numbers: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let pattern = if args.ignore_case {
        format!("(?i){}", &args.pattern)
    } else {
        args.pattern.clone()
    };

    let regex = Regex::new(&pattern).context("Failed to create regex pattern")?;

    if args.recursive {
        search_dir(&args.path, &regex, &args)?
    } else {
        search_file(&args.path, &regex, &args)?
    }

    Ok(())
}

fn search_file(path: &PathBuf, regex: &Regex, args: &Args) -> Result<()> {
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;
    let reader = BufReader::new(file);

    // calculate actual context sizes
    let before_ctx = args
        .context
        .unwrap_or(0)
        .max(args.before_context.unwrap_or(0));
    let after_ctx = args
        .context
        .unwrap_or(0)
        .max(args.after_context.unwrap_or(0));

    let mut previous_lines: VecDeque<(usize, String)> = VecDeque::new();
    let mut print_after = 0;
    let mut last_printed: Option<usize> = None;

    // helper to print colored matches
    fn print_line(path: &PathBuf, line: &str, line_num: usize, regex: &Regex, args: &Args) {
        if args.line_numbers {
            print!("{}:{}: ", path.display(), line_num);
        } else {
            print!("{}:", path.display());
        }

        if args.no_color {
            println!("{}", line);
            return;
        }

        let mut last_match = 0;
        let mut matches = regex.find_iter(line).peekable();

        if matches.peek().is_none() {
            println!("{}", line);
            return;
        }

        for m in matches {
            // text before match
            print!("{}", &line[last_match..m.start()]);
            // match in red
            print!("{}", &line[m.start()..m.end()].red());
            last_match = m.end();
        }

        // remaining text after last match
        println!("{}", &line[last_match..]);
    }

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.context("Failed to read line")?;
        let line_num = line_num + 1; // line nums 1-based

        // if we need to print after-context from previous match
        if print_after > 0 {
            print_line(path, &line, line_num, regex, args);
            print_after -= 1;
            last_printed = Some(line_num);
            continue;
        }

        // store line in previous_lines buffer for before-context
        if before_ctx > 0 {
            if previous_lines.len() >= before_ctx {
                previous_lines.pop_front();
            }
            previous_lines.push_back((line_num, line.clone()));
        }

        // check if current line matches
        if regex.is_match(&line) {
            // print separator if this isn't immediately after previous print
            if let Some(last) = last_printed {
                if line_num > last + 1 {
                    println!("--");
                }
            }

            // before-context
            for (num, context_line) in &previous_lines {
                print_line(path, context_line, *num, regex, args);
            }
            previous_lines.clear();

            // matching line
            print_line(path, &line, line_num, regex, args);
            last_printed = Some(line_num);

            // set up after-context
            print_after = after_ctx;
        }
    }

    Ok(())
}

fn search_dir(dir: &PathBuf, regex: &Regex, args: &Args) -> Result<()> {
    use walkdir::WalkDir;

    for entry in WalkDir::new(dir) {
        let entry = entry.context("Failed to access directory entry")?;
        if entry.file_type().is_file() {
            search_file(&entry.path().to_path_buf(), regex, args)?;
        }
    }

    Ok(())
}
