use anyhow::Result;
use cargo_metadata::{MetadataCommand, Metadata, CargoOpt};


pub fn exec() -> Result<Metadata> {
    let metadata = MetadataCommand::new()
        .features(CargoOpt::AllFeatures)
        // .other_options(["--verbose".to_string()])
        .exec()?;
    Ok(metadata)
}