mod cli;
mod core;
mod tui;

fn main() {
    let args = cli::args::parse();
    if args.path.is_none() {
        tui::run();
    } else {
        cli::run(args);
    }
}
