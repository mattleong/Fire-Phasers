use std::process::{Command, exit};
use std::env;

fn get_env_var(var_key: &str) -> String {
    let path = env::vars()
        .filter(|(k, _v)| k == var_key)
        .map(|(_k, v)| v)
        .collect();

    return path;
}

fn main() {
    let dev_path_key = String::from("DT_PATH");
    let dev_path = get_env_var(&dev_path_key);

    if dev_path.is_empty() {
        println!("No path set.");
        exit(0);
    }

    println!("path: {}", dev_path);

    // rake wp:npm -- run build:prod
    Command::new("rake")
        .args(&["wp:npm", "--", "run", "build:prod"])
        .current_dir(dev_path)
        .spawn()
        .expect("failed to execute process");
}
