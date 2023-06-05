use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    HetznerApiActionModel, HetznerApiAppliedToModel, HetznerApiLabelSelector,
    HetznerApiSelectorType, HetznerApiServerAppliedToModel,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CreateFirewallRequestRulesDirectionType {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CreateFirewallRequestRulesProtocolType {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "esp")]
    Esp,
    #[serde(rename = "gre")]
    Gre,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFirewallRequestRulesModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ips: Option<Vec<String>>,
    pub direction: CreateFirewallRequestRulesDirectionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ips: Option<Vec<String>>,
    pub port: Option<String>,
    pub protocol: CreateFirewallRequestRulesProtocolType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFirewallRequestApplyToModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<HetznerApiLabelSelector>,
    pub server: Option<HetznerApiServerAppliedToModel>,
    #[serde(rename = "type")]
    pub selector_type: HetznerApiSelectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFirewallRequest {
    pub apply_to: Vec<CreateFirewallRequestApplyToModel>,
    pub labels: HashMap<String, String>,
    pub name: String,
    pub rules: Vec<CreateFirewallRequestRulesModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerApiFirewallModel {
    pub applied_to: Vec<HetznerApiAppliedToModel>,
    pub created: String,
    pub id: i64,
    pub labels: Option<HashMap<String, String>>,
    pub name: String,
    pub rules: Vec<CreateFirewallRequestRulesModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerCreateFirewallResponse {
    pub actions: Vec<HetznerApiActionModel>,
    pub firewall: HetznerApiFirewallModel,
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        AppliedToResourcesModel, HetznerApiActionResourcesModel, HetznerApiActionStatusType,
        HetznerError, HetznerErrorCode,
    };

    use super::*;

    #[test]
    fn greeting_contains_name() {
        let str = serde_json::to_string(&CreateFirewallRequest {
            apply_to: vec![CreateFirewallRequestApplyToModel {
                label_selector: None,
                server: Some(HetznerApiServerAppliedToModel { id: 42 }),
                selector_type: HetznerApiSelectorType::Server,
            }],
            labels: HashMap::from_iter(vec![("env".to_string(), "dev".to_string())]),
            name: "Corporate Intranet Protection".to_string(),
            rules: vec![CreateFirewallRequestRulesModel {
                description: Some("Allow port 80".to_string()),
                destination_ips: None,
                direction: CreateFirewallRequestRulesDirectionType::In,
                source_ips: Some(vec![
                    "28.239.13.1/32".to_string(),
                    "28.239.14.0/24".to_string(),
                    "ff21:1eac:9a3b:ee58:5ca:990c:8bc9:c03b/128".to_string(),
                ]),
                port: Some("80".to_string()),
                protocol: CreateFirewallRequestRulesProtocolType::Tcp,
            }],
        });

        println!("{}", str.unwrap());
        // assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name2() {
        // let str = serde_json::to_string(&);

        let a = serde_json::to_string(&HetznerCreateFirewallResponse {
            actions: vec![
                HetznerApiActionModel {
                    command: "set_firewall_rules".to_string(),
                    error: Some(HetznerError {
                        code: HetznerErrorCode::ActionFailed,
                        message: "Action failed".to_string(),
                    }),
                    finished: Some("2016-01-30T23:56:00+00:00".to_string()),
                    id: 14,
                    progress: 100,
                    resources: vec![
                        HetznerApiActionResourcesModel {
                            id: 42,
                            resource_type: "server".to_string(),
                        },
                        HetznerApiActionResourcesModel {
                            id: 38,
                            resource_type: "firewall".to_string(),
                        },
                    ],
                    started: "2016-01-30T23:55:00+00:00".to_string(),
                    status: HetznerApiActionStatusType::Success,
                },
                HetznerApiActionModel {
                    command: "apply_firewall".to_string(),
                    error: Some(HetznerError {
                        code: HetznerErrorCode::ActionFailed,
                        message: "Action failed".to_string(),
                    }),
                    finished: Some("2016-01-30T23:56:00+00:00".to_string()),
                    id: 14,
                    progress: 100,
                    resources: vec![
                        HetznerApiActionResourcesModel {
                            id: 42,
                            resource_type: "server".to_string(),
                        },
                        HetznerApiActionResourcesModel {
                            id: 38,
                            resource_type: "firewall".to_string(),
                        },
                    ],
                    started: "2016-01-30T23:55:00+00:00".to_string(),
                    status: HetznerApiActionStatusType::Success,
                },
            ],
            firewall: HetznerApiFirewallModel {
                applied_to: vec![HetznerApiAppliedToModel {
                    applied_to_resources: vec![AppliedToResourcesModel {
                        server: HetznerApiServerAppliedToModel { id: 42 },
                    }],
                    label_selector: Some((&"env=prod".to_string()).into()),
                    server: Some(HetznerApiServerAppliedToModel { id: 42 }),
                    selector_type: HetznerApiSelectorType::Server,
                }],
                created: "2016-01-30T23:55:00+00:00".to_string(),
                id: 42,
                labels: Some(HashMap::new()),
                name: "my-resource".to_string(),
                rules: vec![CreateFirewallRequestRulesModel {
                    description: None,
                    destination_ips: Some(vec![
                        "28.239.13.1/32".to_string(),
                        "28.239.14.0/24".to_string(),
                        "ff21:1eac:9a3b:ee58:5ca:990c:8bc9:c03b/128".to_string(),
                    ]),
                    direction: CreateFirewallRequestRulesDirectionType::In,
                    source_ips: Some(vec![
                        "28.239.13.1/32".to_string(),
                        "28.239.14.0/24".to_string(),
                        "ff21:1eac:9a3b:ee58:5ca:990c:8bc9:c03b/128".to_string(),
                    ]),
                    port: Some("80".to_string()),
                    protocol: CreateFirewallRequestRulesProtocolType::Tcp,
                }],
            },
        });

        println!("{}", a.unwrap());
        // assert!(result.contains("Carol"));
    }
}
