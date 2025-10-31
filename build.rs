use std::io::Result;
fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:warning=OUT_DIR: {}", out_dir);


    let mut config = prost_build::Config::new();
    config.out_dir(std::path::Path::new("src"));
    config.compile_protos(&["src/items.proto"], &["src"])?;
    Ok(()) 
}
