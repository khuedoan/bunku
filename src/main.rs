use bunku::{args::Args, Workload};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let filename = args.filename;
    let filecontent = std::fs::read_to_string(&filename).expect("file not found");

    let app: Workload = toml::from_str(&filecontent).expect("failed to parse workload definition");
    println!("{:#?}", app);

    let deployment = app.clone().deployment();
    let service = app.service();
    println!("{deployment:#?}{service:#?}",);
    println!(
        "{}{}",
        serde_json::to_string(&deployment).unwrap(),
        serde_json::to_string(&service).unwrap()
    );
}
