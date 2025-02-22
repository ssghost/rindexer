use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_yaml::Value;

use crate::{
    indexer::Indexer,
    manifest::{
        contract::Contract, global::Global, graphql::GraphQLSettings, network::Network,
        storage::Storage,
    },
};

fn deserialize_project_type<'de, D>(deserializer: D) -> Result<ProjectType, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => match s.as_str() {
            "rust" => Ok(ProjectType::Rust),
            "no-code" => Ok(ProjectType::NoCode),
            _ => Err(serde::de::Error::custom(format!("Unknown project type: {}", s))),
        },
        _ => Err(serde::de::Error::custom("Invalid project type format")),
    }
}

fn serialize_project_type<S>(value: &ProjectType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_value = match value {
        ProjectType::Rust => "rust",
        ProjectType::NoCode => "no-code",
    };
    serializer.serialize_str(string_value)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ProjectType {
    Rust,
    NoCode,
}

fn default_storage() -> Storage {
    Storage::default()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,

    #[serde(deserialize_with = "deserialize_project_type")]
    #[serde(serialize_with = "serialize_project_type")]
    pub project_type: ProjectType,

    pub networks: Vec<Network>,

    #[serde(default = "default_storage")]
    pub storage: Storage,

    pub contracts: Vec<Contract>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<Global>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graphql: Option<GraphQLSettings>,
}

impl Manifest {
    pub fn to_indexer(&self) -> Indexer {
        Indexer { name: self.name.clone(), contracts: self.contracts.clone() }
    }

    pub fn has_any_contracts_live_indexing(&self) -> bool {
        self.contracts.iter().filter(|c| c.details.iter().any(|p| p.end_block.is_none())).count() >
            0
    }

    pub fn contract_csv_enabled(&self, contract_name: &str) -> bool {
        let contract_csv_enabled = self
            .contracts
            .iter()
            .find(|c| c.name == contract_name)
            .map_or(false, |c| c.generate_csv.unwrap_or(true));

        self.storage.csv_enabled() && contract_csv_enabled
    }
}
