pub mod api_version;
pub mod container;
pub mod metadata;

use api_version::ApiVersion;
use container::Container;
use metadata::Metadata;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deploy {
    pub api_version: ApiVersion,
    pub metadata: Metadata,
    pub containers: HashMap<String, Container>,
}
