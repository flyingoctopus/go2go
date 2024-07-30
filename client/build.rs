// client/build.rs
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("Running build script...");

    // Run wasm-pack build --target web and capture the output
    let output = Command::new("wasm-pack")
        .args(&["build", "--target", "web"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute wasm-pack");

    // Print the standard output of wasm-pack
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Print the standard error of wasm-pack (including warnings)
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Get the OUT_DIR environment variable
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not defined");
    println!("OUT_DIR: {}", out_dir);

    // Navigate to the pkg directory
    let mut dest_path = PathBuf::from(out_dir);
    dest_path.pop(); // Pop the build
    dest_path.pop(); // Pop the target/wasm32-unknown-unknown/release/build

    // Set the destination path to pkg
    let dest_path = dest_path.join("pkg");
    println!("Destination path: {:?}", dest_path);

    // Create the pkg directory if it doesn't exist
    fs::create_dir_all(&dest_path).expect("Failed to create pkg directory");

    // Copy the index.html from static to pkg
    fs::copy("static/index.html", dest_path.join("index.html"))
        .expect("Failed to copy index.html");
    println!("index.html copied successfully!");
}
