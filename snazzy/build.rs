use std::io::Result;

fn main () -> Result<()>
{
    println!("build.rs : hello");
    // prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}