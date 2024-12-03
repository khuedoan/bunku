use bunku::Deploy;

fn main() {
    let example = std::fs::read_to_string("examples/hello-world/deploy.toml").unwrap();
    let deploy: Deploy = toml::from_str(&example).unwrap();
    println!("{:#?}", deploy);
}
