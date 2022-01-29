use anyhow::Result;
use cargo_metadata::{MetadataCommand, Metadata, CargoOpt};
use std::{env, fs};
use cargo_toml::{Manifest, OptionalFile};


pub fn exec() -> Result<Metadata> {
    let metadata = MetadataCommand::new()
        .features(CargoOpt::AllFeatures)
        // .other_options(["--verbose".to_string()])
        .exec()?;
    Ok(metadata)
}

pub fn readme() -> Result<String> {
    let readme_file = env::current_dir()?.join(readme_file()?);
    let readme = fs::read_to_string(readme_file).unwrap_or("no readme".into());
    Ok(readme)
}

pub fn readme_file() -> Result<String> {
    if fs::metadata("Cargo.toml").is_ok() {
        let m = Manifest::from_path("Cargo.toml")?;
        if let Some(OptionalFile::Path(readme)) = m.package.map(|p| p.readme) {
            return Ok(readme);
        }
    }
    Ok("README.md".into())
}
