use crate::{CreateFirewallRequestApplyToModel, HetznerApiSelectorType, CreateFirewallRequestRulesModel, CreateFirewallRequestRulesProtocolType, CreateFirewallRequestRulesDirectionType, NetworkSubnetRequestModel, CreateNetworkRequestPublicNetModel};

pub enum HetznerClientApplyTo {
    Labels(String),
    Server(i64),
}

impl Into<CreateFirewallRequestApplyToModel> for &HetznerClientApplyTo {
    fn into(self) -> CreateFirewallRequestApplyToModel {
        match self {
            HetznerClientApplyTo::Labels(label) => CreateFirewallRequestApplyToModel {
                label_selector: Some(label.into()),
                server: None,
                selector_type: HetznerApiSelectorType::LabelSelector,
            },
            HetznerClientApplyTo::Server(id) => CreateFirewallRequestApplyToModel {
                label_selector: None,
                server: Some(id.clone().into()),
                selector_type: HetznerApiSelectorType::Server,
            },
        }
    }
}


pub struct HetznerClientFirewallRules {
    pub destination_ips: Option<Vec<String>>,
    pub direction: CreateFirewallRequestRulesDirectionType,
    pub source_ips: Option<Vec<String>>,
    pub port: Option<String>,
    pub protocol: CreateFirewallRequestRulesProtocolType,
    pub description: Option<String>,
}


impl Into<CreateFirewallRequestRulesModel> for &HetznerClientFirewallRules {
    fn into(self) -> CreateFirewallRequestRulesModel {
        CreateFirewallRequestRulesModel{
            description: self.description.clone(),
            destination_ips: self.destination_ips.clone(),
            direction: self.direction.clone(),
            source_ips: self.source_ips.clone(),
            port: self.port.clone(),
            protocol: self.protocol.clone(),
        }
    }
}

pub struct HetznerClientSubnetModel{
    pub ip_range: String,
    pub network_zone: String
}

impl Into<NetworkSubnetRequestModel> for &HetznerClientSubnetModel {
    fn into(self) -> NetworkSubnetRequestModel {
        NetworkSubnetRequestModel{
            ip_range: self.ip_range.clone(),
            network_zone: self.network_zone.clone(),
            network_type: "cloud".to_string(),
        }
    }
}

pub enum HetznerClientPublicNetType{
    IPv4(Option<i64>),
    IPv6(Option<i64>),
    Both{
        ipv4: Option<i64>,
        ipv6: Option<i64>,
    }

}

impl Into<CreateNetworkRequestPublicNetModel> for HetznerClientPublicNetType {
    fn into(self) -> CreateNetworkRequestPublicNetModel {

        let mut network_conf = CreateNetworkRequestPublicNetModel{
            enable_ipv4: false,
            enable_ipv6: false,
            ipv4: None,
            ipv6: None,
        };

        match self{
            HetznerClientPublicNetType::IPv4(ip) => {
                network_conf.enable_ipv4 = true;
                network_conf.ipv4 = ip;
            },
            HetznerClientPublicNetType::IPv6(ip) => {
                network_conf.enable_ipv6 = true;
                network_conf.ipv6 = ip;
            },
            HetznerClientPublicNetType::Both { ipv4, ipv6 } => {
                network_conf.enable_ipv4 = true;
                network_conf.enable_ipv6 = true;
                network_conf.ipv4 = ipv4;
                network_conf.ipv6 = ipv6;
            },
        };

        return network_conf;
    }
}