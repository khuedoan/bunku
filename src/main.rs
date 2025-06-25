use bunku::process_values_file;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    match process_values_file(&args.filename) {
        Ok(resources) => {
            let json = serde_json::to_string_pretty(&resources).unwrap();
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
