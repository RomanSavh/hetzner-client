use serde::{Deserialize, Serialize};

use crate::HetznerError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HetznerApiLabelSelector {
    pub selector: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HetznerApiSelectorType {
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "label_selector")]
    LabelSelector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerApiServerAppliedToModel {
    pub id: i64,
}

impl Into<HetznerApiServerAppliedToModel> for i64 {
    fn into(self) -> HetznerApiServerAppliedToModel {
        HetznerApiServerAppliedToModel { id: self }
    }
}

impl Into<HetznerApiLabelSelector> for &String {
    fn into(self) -> HetznerApiLabelSelector {
        HetznerApiLabelSelector { selector: self.clone() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerApiActionResourcesModel {
    pub id: i64,
    #[serde(rename = "type")]
    pub resource_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HetznerApiActionStatusType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerApiActionModel {
    pub command: String,
    pub error: Option<HetznerError>,
    pub finished: Option<String>,
    pub id: i64,
    pub progress: i32,
    pub resources: Vec<HetznerApiActionResourcesModel>,
    pub started: String,
    pub status: HetznerApiActionStatusType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppliedToResourcesModel {
    pub server: HetznerApiServerAppliedToModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerApiAppliedToModel {
    pub applied_to_resources: Vec<AppliedToResourcesModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<HetznerApiLabelSelector>,
    pub server: Option<HetznerApiServerAppliedToModel>,
    #[serde(rename = "type")]
    pub selector_type: HetznerApiSelectorType,
}
