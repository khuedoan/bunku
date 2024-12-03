use bunku::Deploy;

fn main() {
    let example = std::fs::read_to_string("examples/hello-world/deploy.toml").unwrap();
    let deploy: Deploy = toml::from_str(&example).unwrap();
    let deployment: k8s_openapi::api::apps::v1::Deployment = deploy.into();
    println!("{:#?}", deployment);
}
