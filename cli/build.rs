use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_enabled = env::var("BUILD_ENABLED").map(|v| v == "1").unwrap_or(false); // run by default
    if build_enabled {
        #[cfg(feature = "build")]
        tonic_build::configure()
            .out_dir("./src/api")
            .compile(&["./proto/exec.proto"], &["./proto"])?;
    }
    Ok(())
}
