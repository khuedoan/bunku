use bunku::{args::Args, Workload};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let filename = args.filename;
    let filecontent = std::fs::read_to_string(&filename).expect("file not found");

    let app: Workload = toml::from_str(&filecontent).expect("failed to parse workload definition");
    let mut kube_manifests: Vec<bunku::resource::Manifest> = app
        .clone()
        .resources
        .unwrap_or_default()
        .into_iter()
        .flat_map(|(id, resource)| resource.provision(app.metadata.clone(), id))
        .collect();

    kube_manifests.push(bunku::resource::Manifest::Deployment(
        app.clone().deployment(),
    ));

    match app.service() {
        Some(service) => kube_manifests.push(bunku::resource::Manifest::Service(service)),
        None => {}
    }

    println!("{:#?}", kube_manifests);
}
