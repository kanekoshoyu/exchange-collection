use cargo_toml::Manifest;
use eyre::Result;
use std::path::{Path, PathBuf};

use super::input::*;

#[derive(Clone, Debug)]
pub struct ProtocolCrate {
    pub protocol: Protocol,
    pub version: Version,
}
impl TryFrom<Manifest> for ProtocolCrate {
    type Error = eyre::Error;

    fn try_from(manifest: Manifest) -> std::result::Result<Self, Self::Error> {
        let Some(protocol_str) = manifest.package().name().split("-").last() else {
            return Err(eyre::eyre!("failed parsing protocol name"));
        };
        let version_str = manifest.package().version();
        Ok(ProtocolCrate {
            protocol: protocol_str.parse()?,
            version: version_str.parse()?,
        })
    }
}
impl ProtocolCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let cargo_toml_path = path.join(Path::new("Cargo.toml"));
        let Ok(manifest) = Manifest::from_path(cargo_toml_path) else {
            panic!("failed reading manifest from path");
        };
        ProtocolCrate::try_from(manifest)
    }
}
#[derive(Clone, Debug, Default)]
pub struct ExchangeCrate {
    pub exchange_name: String,
    pub version: Version,
    pub protocol_crates: Vec<ProtocolCrate>,
}
impl ExchangeCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let src_path = path.join(PathBuf::from("src"));

        let protocol_dirs: Vec<PathBuf> = std::fs::read_dir(src_path)?
            .map(|item| item.unwrap().path())
            .filter(|path| path.is_dir())
            .collect();

        let mut protocol_crates = Vec::new();
        for protocol_dir in protocol_dirs {
            protocol_crates.push(ProtocolCrate::from_path(protocol_dir)?);
        }

        let version = protocol_crates
            .iter()
            .cloned()
            .fold(Version::default(), |acc, protocol_crate| {
                acc + protocol_crate.version
            });

        let exchange_name: String = path
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap_or_default();

        Ok(ExchangeCrate {
            exchange_name,
            version,
            protocol_crates,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CollectionCrate {
    pub version: Version,
    pub exchange_crates: Vec<ExchangeCrate>,
}
impl CollectionCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let src_path = path.join(PathBuf::from("src"));

        let exchange_dirs: Vec<PathBuf> = std::fs::read_dir(src_path)?
            .map(|item| item.unwrap().path())
            .filter(|path| path.is_dir())
            .collect();

        let mut exchange_crates = Vec::new();
        for exchange_dir in exchange_dirs {
            exchange_crates.push(ExchangeCrate::from_path(exchange_dir)?);
        }

        let version = exchange_crates
            .iter()
            .cloned()
            .fold(Version::default(), |acc, protocol_crate| {
                acc + protocol_crate.version
            });

        Ok(CollectionCrate {
            version,
            exchange_crates,
        })
    }
}
