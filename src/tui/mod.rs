use crate::cli::{self, args::Args};
use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Editor};
use std::env;
use std::path::PathBuf;

struct PathHelper {
    completer: FilenameCompleter,
}

impl rustyline::Helper for PathHelper {}
impl rustyline::completion::Completer for PathHelper {
    type Candidate = rustyline::completion::Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for PathHelper {
    type Hint = String;
}
impl Highlighter for PathHelper {}
impl Validator for PathHelper {}

fn history_file(name: &str) -> Option<PathBuf> {
    env::var("HOME").ok().map(|h| PathBuf::from(h).join(name))
}

// Simple readline with no history — used for yes/no prompts where history adds no value
fn read_raw(prompt: &str) -> String {
    let mut rl = DefaultEditor::new().unwrap();
    match rl.readline(prompt) {
        Ok(line) => line.trim().to_string(),
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => std::process::exit(0),
        Err(e) => panic!("Input error: {e}"),
    }
}

fn read_line(prompt: &str) -> String {
    let hist = history_file(".gopro_renamer_history");
    let mut rl = DefaultEditor::new().unwrap();
    if let Some(ref h) = hist {
        let _ = rl.load_history(h);
    }
    let line = match rl.readline(prompt) {
        Ok(line) => line.trim().to_string(),
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => std::process::exit(0),
        Err(e) => panic!("Input error: {e}"),
    };
    let _ = rl.add_history_entry(&line);
    if let Some(ref h) = hist {
        let _ = rl.save_history(h);
    }
    line
}

// Returns None if the user presses Enter (meaning: use current directory)
fn read_path(prompt: &str) -> Option<PathBuf> {
    let hist = history_file(".gopro_renamer_paths");
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let helper = PathHelper {
        completer: FilenameCompleter::new(),
    };
    let mut rl: Editor<PathHelper, _> = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));
    if let Some(ref h) = hist {
        let _ = rl.load_history(h);
    }
    let line = match rl.readline(prompt) {
        Ok(line) => line.trim().to_string(),
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => std::process::exit(0),
        Err(e) => panic!("Input error: {e}"),
    };
    if !line.is_empty() {
        let _ = rl.add_history_entry(&line);
    }
    if let Some(ref h) = hist {
        let _ = rl.save_history(h);
    }
    if line.is_empty() {
        None
    } else {
        Some(PathBuf::from(line))
    }
}

fn read_bool(prompt: &str) -> bool {
    loop {
        match read_raw(prompt).to_lowercase().as_str() {
            "true" | "yes" | "y" => return true,
            "false" | "no" | "n" | "" => return false,
            _ => println!("{}", "  Please enter y/n".red().bold()),
        }
    }
}

pub fn run() {
    let version = env!("CARGO_PKG_VERSION");
    println!("\n{}", format!("  GoPro Renamer  v{version}").bold().cyan());
    println!("{}\n", "  ─────────────────────".dimmed());

    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| String::from("."));

    let path = read_path(&format!(
        "{} {}: ",
        "  path".yellow().bold(),
        format!("[Enter for {cwd}]").dimmed()
    ));

    let dry_run = read_bool(&format!("{}: ", "  dry run? (y/n)".yellow().bold()));
    let concatenate = read_bool(&format!(
        "{}: ",
        "  concatenate chapters? (y/n)".yellow().bold()
    ));

    let prefix = if !concatenate {
        read_line(&format!(
            "{} {}: ",
            "  prefix".yellow().bold(),
            "[Enter to skip]".dimmed()
        ))
    } else {
        String::new()
    };

    // Summary before running
    let display_path = path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| cwd.clone());

    println!(
        "\n{}",
        "  ─────────────────────────────────────────".dimmed()
    );
    println!("  {}  {}", "path:       ".dimmed(), display_path.bold());
    println!(
        "  {}  {}",
        "dry run:    ".dimmed(),
        if dry_run {
            "yes".yellow().bold()
        } else {
            "no".bold()
        }
    );
    println!(
        "  {}  {}",
        "concatenate:".dimmed(),
        if concatenate {
            "yes".green().bold()
        } else {
            "no".bold()
        }
    );
    if !concatenate {
        let prefix_disp = if prefix.is_empty() {
            "(none)".dimmed().to_string()
        } else {
            prefix.bold().to_string()
        };
        println!("  {}  {}", "prefix:     ".dimmed(), prefix_disp);
    }
    println!("{}", "  ─────────────────────────────────────────".dimmed());

    if !read_bool(&format!("\n{}: ", "  proceed? (y/n)".yellow().bold())) {
        println!("{}", "\n  Cancelled.".yellow());
        return;
    }

    println!();

    let args = Args {
        path,
        dry_run,
        prefix,
        concatenate,
    };
    cli::run(args);
}
