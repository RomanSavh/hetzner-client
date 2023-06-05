use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNetworkRequest {
    pub ip_range: String,
    pub labels: HashMap<String, String>,
    pub name: String,
    pub routes: Vec<CreateNetworkRouteModel>,
    pub subnets: Vec<NetworkSubnetRequestModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNetworkRouteModel {
    pub destination: String,
    pub gateway: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkSubnetRequestModel {
    pub ip_range: String,
    pub network_zone: String,
    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkSubnetModel {
    pub ip_range: String,
    pub network_zone: String,
    #[serde(rename = "type")]
    pub network_type: String,
    pub gateway: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNetworkResponse {
    pub network: HetznerNetworkModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerNetworkRoutesModel {
    pub destination: String,
    pub gateway: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HetznerNetworkModel {
    pub created: String,
    pub id: i64,
    pub ip_range: String,
    pub labels: HashMap<String, String>,
    pub load_balancers: Vec<i64>,
    pub name: String,
    pub routes: Vec<HetznerNetworkRoutesModel>,
    pub servers: Vec<i64>,
    pub subnets: Vec<NetworkSubnetModel>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let str = serde_json::to_string(&CreateNetworkRequest {
            ip_range: "10.0.0.0/16".to_string(),
            labels: HashMap::from_iter(vec![("labelkey".to_string(), "value".to_string())]),
            name: "mynet".to_string(),
            routes: vec![CreateNetworkRouteModel {
                destination: "10.100.1.0/24".to_string(),
                gateway: "10.0.1.1".to_string(),
            }],
            subnets: vec![NetworkSubnetRequestModel {
                ip_range: "10.0.1.0/24".to_string(),
                network_zone: "eu-central".to_string(),
                network_type: "cloud".to_string(),
            }],
        });

        println!("{}", str.unwrap());
        // assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_2() {
        let str = serde_json::to_string(&CreateNetworkResponse {
            network: HetznerNetworkModel {
                created: "2016-01-30T23:50:00+00:00".to_string(),
                id: 4711,
                ip_range: "10.0.0.0/16".to_string(),
                labels: HashMap::new(),
                load_balancers: vec![42],
                name: "mynet".to_string(),
                routes: vec![HetznerNetworkRoutesModel {
                    destination: "10.100.1.0/24".to_string(),
                    gateway: "10.0.1.1".to_string(),
                }],
                servers: vec![42],
                subnets: vec![NetworkSubnetModel {
                    ip_range: "10.0.1.0/24".to_string(),
                    network_zone: "eu-central".to_string(),
                    network_type: "cloud".to_string(),
                    gateway: "10.0.0.1".to_string(),
                }],
            },
        });

        println!("{}", str.unwrap());
        // assert!(result.contains("Carol"));
    }
}
