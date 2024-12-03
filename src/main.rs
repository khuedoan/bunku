use bunku::App;

fn main() {
    let example = std::fs::read_to_string("examples/hello-world/app.toml").unwrap();
    let app: App = toml::from_str(&example).unwrap();
    let deployment: k8s_openapi::api::apps::v1::Deployment = app.into();
    let json = serde_json::to_string_pretty(&deployment).unwrap();
    println!("{}", json);
}
