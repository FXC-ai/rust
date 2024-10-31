use std::io::Result;
fn main() -> Result<()>
{
    let mut config = prost_build::Config::new();

    config.out_dir("src");
    config.compile_protos(&["src/items.proto"], &["src/"])?;

    Ok(())
}