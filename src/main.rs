use bunku::process_values_file;
use clap::Parser;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(
        short,
        long,
        help = "Output directory for separate JSON files (optional)"
    )]
    output_dir: Option<String>,

    #[arg(short, long, help = "App name to prepend to resource names")]
    name: Option<String>,
}

fn get_resource_filename(resource: &Value) -> String {
    let kind = resource["kind"].as_str().unwrap_or("unknown");
    let name = resource["metadata"]["name"].as_str().unwrap_or("unnamed");

    format!("{}-{}.json", kind, name)
}

fn create_list_object(items: Vec<Value>) -> Value {
    serde_json::json!({
        "apiVersion": "v1",
        "kind": "List",
        "items": items
    })
}

fn main() {
    let args = Args::parse();

    match process_values_file(&args.filename) {
        Ok(mut resources) => {
            // Apply name prefix if provided
            if let Some(app_name) = &args.name {
                for resource in &mut resources {
                    if let Some(metadata) = resource.get_mut("metadata") {
                        if let Some(name) = metadata.get("name").and_then(|n| n.as_str()) {
                            let new_name = format!("{}-{}", app_name, name);
                            metadata["name"] = serde_json::Value::String(new_name);
                        }
                    }
                }
            }

            match args.output_dir {
                Some(output_dir) => {
                    // Create output directory if it doesn't exist
                    if let Err(e) = fs::create_dir_all(&output_dir) {
                        eprintln!("Error creating output directory: {}", e);
                        std::process::exit(1);
                    }

                    // Write each resource to a separate file
                    for resource in &resources {
                        let filename = get_resource_filename(resource);
                        let filepath = Path::new(&output_dir).join(&filename);
                        let json = serde_json::to_string_pretty(resource).unwrap();

                        if let Err(e) = fs::write(&filepath, json) {
                            eprintln!("Error writing file {}: {}", filepath.display(), e);
                            std::process::exit(1);
                        }

                        println!("Wrote {}", filepath.display());
                    }
                }
                None => {
                    // Wrap resources in a List object for stdout
                    let list = create_list_object(resources);
                    let json = serde_json::to_string_pretty(&list).unwrap();
                    println!("{}", json);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
