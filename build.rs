fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .file("./schema/node.capnp")
        .run()?;
    Ok(())
}
