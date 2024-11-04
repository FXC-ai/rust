use std::io::Result;
fn main() -> Result<()>
{
    let mut config = prost_build::Config::new();

    config.out_dir("src");
    config.compile_protos(&["src/items.proto"], &["src/"])?;

    let mut config2 = prost_build::Config::new();

    config2.out_dir("src");
    config2.compile_protos(&["src/polym.proto"], &["src/"])?;

    Ok(())
}