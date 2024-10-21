use prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["proto/command.proto"], &["proto/"])?;
    Ok(())
}
