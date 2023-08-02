fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src/api")
        .compile(&["./proto/exec.proto"], &["./proto"])?;
    Ok(())
}
