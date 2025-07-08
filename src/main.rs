use clap::Parser;

mod cli;
mod utils;
mod work;

fn main() {
    let arg = cli::Args::parse();
    match arg.command {
        cli::Commands::Init { terminal, options } => {
            let result = work::init(terminal, options.query, options.r#override);
            match options.output {
                Some(file) => std::fs::write(file, result).expect("Failed to write to file"),
                None => print!("{result}"),
            }
        }
        cli::Commands::Cat => {
            let result = work::cat();
            print!("{result}");
        }
    };
}
