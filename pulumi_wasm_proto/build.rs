use std::env::var_os;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_location = format!("{}/mini", var_os("OUT_DIR").unwrap().to_str().unwrap());
    fs::create_dir_all(&model_location)?;
    let full_location = format!("{}/full", var_os("OUT_DIR").unwrap().to_str().unwrap());
    fs::create_dir_all(&full_location)?;

    tonic_build::configure()
        .build_transport(false)
        .build_client(false)
        .build_server(false)
        .out_dir(model_location)
        .compile(
            &[
                "proto/pulumi/plugin.proto",
                "proto/pulumi/engine.proto",
                "proto/pulumi/resource.proto",
            ],
            &["proto"],
        )?;

    tonic_build::configure()
        .build_transport(true)
        .build_client(true)
        .build_server(true)
        .out_dir(full_location)
        .compile(
            &[
                "proto/pulumi/plugin.proto",
                "proto/pulumi/engine.proto",
                "proto/pulumi/resource.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
