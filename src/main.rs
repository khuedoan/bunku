use bunku::App;

fn main() {
    let example = std::fs::read_to_string("examples/nginx/app.toml").unwrap();
    let app: App = toml::from_str(&example).unwrap();
    let deployment = app.clone().deployment();
    let service = app.service();
    println!(
        "{}{}",
        serde_json::to_string(&deployment).unwrap(),
        serde_json::to_string(&service).unwrap()
    );
}
