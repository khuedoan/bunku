use bunku::{args::Args, App};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let filename = args.filename;
    let filecontent = std::fs::read_to_string(&filename).unwrap();

    let app: App = toml::from_str(&filecontent).unwrap();
    println!("{:#?}", app);

    let deployment = app.clone().deployment();
    let service = app.service();
    println!(
        "{}{}",
        serde_json::to_string(&deployment).unwrap(),
        serde_json::to_string(&service).unwrap()
    );
}
