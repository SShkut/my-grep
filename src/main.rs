use clap::{Parser, Subcommand};
use my_grep::input::{find_in_file, find_in_std_in};

#[derive(Parser, Debug)]
#[command(name = "my-grep", version)]
struct Cli {
    #[command(subcommand)]
    input: Option<Input>,

    search_pattern: String,
}

#[derive(Subcommand, Debug)]
enum Input {
    #[command(short_flag = 'f', arg_required_else_help = true)]
    File { file_name: String },
}

fn main() {
    let args = Cli::parse();

    match &args.input {
        Some(Input::File { file_name }) => find_in_file(&file_name, &args.search_pattern),
        None => find_in_std_in(&args.search_pattern),
    }
}
