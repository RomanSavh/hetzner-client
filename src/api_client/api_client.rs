use std::collections::HashMap;

use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

use crate::{
    CreateFirewallRequest, CreateNetworkRequest, CreateNetworkRequestFirewallModel,
    CreateNetworkResponse, CreateNetworkRouteModel, CreateServerRequest, CreateServerResponse,
    HetznerClientApplyTo, HetznerClientFirewallRules, HetznerClientPublicNetType,
    HetznerClientSubnetModel, HetznerCreateFirewallResponse, ServerInfoResponse,
};

pub struct ApiClient {
    pub api_key: String,
    pub base_url: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> ApiClient {
        ApiClient {
            api_key,
            base_url: "https://api.hetzner.cloud/v1".to_string(),
        }
    }
}

impl ApiClient {
    pub async fn create_firewall(
        &self,
        apply_to: Vec<HetznerClientApplyTo>,
        labels: HashMap<String, String>,
        name: String,
        rules: Vec<HetznerClientFirewallRules>,
    ) -> HetznerCreateFirewallResponse {
        let request = CreateFirewallRequest {
            apply_to: apply_to.iter().map(|x| x.into()).collect(),
            labels,
            name,
            rules: rules.iter().map(|x| x.into()).collect(),
        };

        let url = format!("{}/firewalls", self.base_url);

        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.to_string()),
            )
            .header("Content-Type", "application/json");

        let body = Body::from(serde_json::to_string(&request).unwrap());

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(body).unwrap();
        let response = client.request(req).await.unwrap();

        let body = get_body(response).await;

        let str: Result<HetznerCreateFirewallResponse, serde_json::Error> =
            serde_json::from_slice(&body[..]);
        let response = str.unwrap();
        return response;
    }

    pub async fn create_network(
        &self,
        ip_range: String,
        labels: HashMap<String, String>,
        name: String,
        routes: Vec<(String, String)>,
        subnets: Vec<HetznerClientSubnetModel>,
    ) -> CreateNetworkResponse {
        let request = CreateNetworkRequest {
            ip_range,
            labels,
            name,
            routes: routes
                .iter()
                .map(|(dest, gateway)| CreateNetworkRouteModel {
                    destination: dest.clone(),
                    gateway: gateway.clone(),
                })
                .collect(),
            subnets: subnets.iter().map(|x| x.into()).collect(),
        };

        let url = format!("{}/networks", self.base_url);

        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.to_string()),
            )
            .header("Content-Type", "application/json");

        let body = Body::from(serde_json::to_string(&request).unwrap());

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(body).unwrap();
        let response = client.request(req).await.unwrap();

        let body = get_body(response).await;


        let str: Result<CreateNetworkResponse, serde_json::Error> =
            serde_json::from_slice(&body[..]);

        let response = str.unwrap();

        response
    }

    pub async fn create_vm(
        &self,
        firewals_ids: Vec<i64>,
        image: String,
        location: Option<String>,
        datacenter: Option<String>,
        networks: Vec<i64>,
        labels: HashMap<String, String>,
        name: String,
        public_net: HetznerClientPublicNetType,
        server_type: String,
        ssh_keys: Vec<String>,
        volumes: Vec<i64>,
    ) -> CreateServerResponse {
        let request = CreateServerRequest {
            automount: false,
            datacenter,
            firewalls: firewals_ids
                .iter()
                .map(|x| CreateNetworkRequestFirewallModel {
                    firewall: x.to_owned(),
                })
                .collect(),
            image,
            labels,
            location,
            name,
            networks,
            placement_group: None,
            public_net: public_net.into(),
            server_type,
            ssh_keys,
            start_after_create: true,
            user_data: "".to_string(),
            volumes,
        };

        let url = format!("{}/servers", self.base_url);

        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.to_string()),
            )
            .header("Content-Type", "application/json");



        let body = Body::from(serde_json::to_string(&request).unwrap());

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(body).unwrap();
        let response = client.request(req).await.unwrap();

        let body = get_body(response).await;

        let str: Result<CreateServerResponse, serde_json::Error> =
            serde_json::from_slice(&body[..]);

        let response = str.unwrap();

        response
    }

    pub async fn get_vm_info(&self, vm_id: i64) -> ServerInfoResponse {
        let url = format!("{}/servers/{}", self.base_url, vm_id);

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.to_string()),
            )
            .header("Content-Type", "application/json");

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let req = req.body(Body::empty()).unwrap();
        let response = client.request(req).await.unwrap();

        let body = get_body(response).await;

        let str: Result<ServerInfoResponse, serde_json::Error> = serde_json::from_slice(&body[..]);

        return str.unwrap();
    }
}

async fn get_body(response: Response<Body>) -> Vec<u8> {
    let body = response.into_body();
    let full_body = hyper::body::to_bytes(body).await.unwrap();
    full_body.iter().cloned().collect::<Vec<u8>>()
}
