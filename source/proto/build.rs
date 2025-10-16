fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.include_file("proto.rs").compile_protos(
        &[
            "../../proto/v1/auth/get_public_key.proto",
            "../../proto/v1/auth/rotate_keypairs.proto",
            "../../proto/v1/auth/signup.proto",
            "../../proto/v1/auth/signin.proto",
            "../../proto/v1/auth/refresh.proto",
        ],
        &["../../proto"],
    )?;
    Ok(())
}
