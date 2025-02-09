use cargo_toml::{Dependency, DependencyDetail, Manifest, Resolver, Workspace};
use eyre::Result;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use super::input::*;

#[derive(Clone, Debug)]
pub struct ExchangeProtocolCrate {
    pub exchange: String,
    pub protocol: Protocol,
    pub version: Version,
}
impl TryFrom<Manifest> for ExchangeProtocolCrate {
    type Error = eyre::Error;

    fn try_from(manifest: Manifest) -> std::result::Result<Self, Self::Error> {
        let crate_name = manifest.package().name();
        let tokens = crate_name.split("-").collect::<Vec<_>>();
        let exchange = tokens[2];
        let protocol = tokens[3];
        let version_str = manifest.package().version();
        Ok(ExchangeProtocolCrate {
            exchange: exchange.parse()?,
            protocol: protocol.parse()?,
            version: version_str.parse()?,
        })
    }
}
impl ExchangeProtocolCrate {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let path = path.as_ref();
        let cargo_toml_path = path.join(Path::new("Cargo.toml"));
        let manifest = Manifest::from_path(cargo_toml_path)?;
        ExchangeProtocolCrate::try_from(manifest)
    }
    // binance_rest
    pub fn exchange_protocol(&self) -> String {
        format!("{}_{}", self.exchange, self.protocol)
    }
}

#[derive(Clone, Debug)]
pub struct CollectionWorkspace {
    pub protocol_crates: Vec<ExchangeProtocolCrate>,
}
impl CollectionWorkspace {
    // this is for bottom up code generation where the srcs are already generated
    pub fn from_src_path(workspace_src_path: impl AsRef<Path>) -> Result<Self> {
        // read Cargo.toml and fill in the data
        let exchange_protocol_dirs: Vec<PathBuf> = std::fs::read_dir(workspace_src_path)?
            .map(|item| item.unwrap().path())
            .filter(|path| path.is_dir())
            .collect();

        let mut protocol_crates = Vec::new();
        for dir in exchange_protocol_dirs {
            let Ok(exchange_protocol_crate) = ExchangeProtocolCrate::from_path(&dir) else {
                println!(
                    "skipped faulty crate from {}",
                    dir.to_str().unwrap_or_default()
                );
                continue;
            };
            protocol_crates.push(exchange_protocol_crate);
        }

        Ok(CollectionWorkspace { protocol_crates })
    }

    pub fn to_workspace(&self) -> Workspace {
        let members = self
            .protocol_crates
            .iter()
            .map(|i: &ExchangeProtocolCrate| format!("src/{}", i.exchange_protocol()))
            .collect();

        let dependencies = {
            let mut dependencies = BTreeMap::new();
            dependencies.insert(
                "serde".to_string(),
                Dependency::Detailed(Box::new(DependencyDetail {
                    version: Some("1.0.217".to_string()),
                    features: vec!["derive".to_string()],
                    ..Default::default()
                })),
            );
            dependencies.insert(
                "serde_json".to_string(),
                Dependency::Simple("1.0.138".to_string()),
            );
            dependencies.insert(
                "serde_yaml".to_string(),
                Dependency::Simple("0.9.33".to_string()),
            );
            dependencies.insert("url".to_string(), Dependency::Simple("2.5.4".to_string()));
            dependencies.insert(
                "reqwest".to_string(),
                Dependency::Simple("0.12.12".to_string()),
            );
            dependencies.insert(
                "tokio-tungstenite".to_string(),
                Dependency::Simple("0.24.0".to_string()),
            );
            dependencies.insert(
                "typed-websocket".to_string(),
                Dependency::Simple("0.1.0".to_string()),
            );
            dependencies.insert(
                "tokio".to_string(),
                Dependency::Detailed(Box::new(DependencyDetail {
                    version: Some("1.43.0".to_string()),
                    features: vec!["full".to_string(), "tracing".to_string()],
                    ..Default::default()
                })),
            );

            dependencies
        };

        // Constructing the Workspace struct
        Workspace {
            members,
            metadata: None,
            package: None,
            resolver: Some(Resolver::V2),
            default_members: Vec::new(),
            exclude: Default::default(),
            dependencies, // ✅ Properly structured dependencies
            lints: Default::default(),
        }
    }

    pub fn to_manifest(&self) -> Manifest {
        // we need to define Manifest anyways
        #[allow(deprecated)]
        Manifest {
            package: None,
            workspace: Some(self.to_workspace()),
            dependencies: Default::default(),
            dev_dependencies: Default::default(),
            build_dependencies: Default::default(),
            target: Default::default(),
            features: Default::default(),
            replace: Default::default(),
            patch: Default::default(),
            lib: Default::default(),
            profile: Default::default(),
            badges: Default::default(),
            bin: Default::default(),
            bench: Default::default(),
            test: Default::default(),
            example: Default::default(),
            lints: Default::default(),
        }
    }
}
