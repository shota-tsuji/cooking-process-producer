fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = tonic_prost_build::compile_protos("../proto/cooking/v1/process.proto");
    if let Err(e) = result {
        eprintln!("Failed to compile protos: {:#?}", e);
        std::process::exit(1);
    }
    Ok(())
}
