use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::HetznerApiActionModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNetworkRequestFirewallModel {
    pub firewall: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNetworkRequestPublicNetModel {
    pub enable_ipv4: bool,
    pub enable_ipv6: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateServerRequest {
    pub automount: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    pub firewalls: Vec<CreateNetworkRequestFirewallModel>,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub location: Option<String>,
    pub name: String,
    pub networks: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<i64>,
    pub public_net: CreateNetworkRequestPublicNetModel,
    pub server_type: String,
    pub ssh_keys: Vec<String>,
    pub start_after_create: bool,
    pub user_data: String,
    pub volumes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerServerModel {
    pub backup_window: Option<String>,
    pub created: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateServerResponse {
    pub action: HetznerApiActionModel,
    pub next_actions: Vec<HetznerApiActionModel>,
    pub root_password: Option<String>,
    pub server: ServerInfoResponseServerModel,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoResponse {
    pub server: ServerInfoResponseServerModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoResponseServerModel{
    pub id: i64,
    pub private_net: Vec<ServerInfoResponsePrivateNetworkModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfoResponsePrivateNetworkModel{
    pub ip: String,
    pub alias_ips: Vec<String>,
    pub mac_address: String,
    pub network: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let str = serde_json::to_string(&CreateServerRequest {
            automount: false,
            datacenter: Some("nbg1-dc3".to_string()),
            firewalls: vec![CreateNetworkRequestFirewallModel { firewall: 38 }],
            image: "ubuntu-20.04".to_string(),
            labels: HashMap::new(),
            location: Some("nbg1".to_string()),
            name: "my-server".to_string(),
            networks: vec![456],
            placement_group: Some(1),
            public_net: CreateNetworkRequestPublicNetModel {
                enable_ipv4: false,
                enable_ipv6: false,
                ipv4: None,
                ipv6: None,
            },
            server_type: "cx11".to_string(),
            ssh_keys: vec!["my-ssh-key".to_string()],
            start_after_create: true,
            user_data: "#cloud-config\nruncmd:\n- [touch, /root/cloud-init-worked]\n".to_string(),
            volumes: vec![123],
        });

        println!("{}", str.unwrap());
        // assert!(result.contains("Carol"));
    }
}
