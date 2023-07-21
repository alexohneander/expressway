use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub global_configuration: GlobalConfiguration,
    pub routes: Vec<Route>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfiguration {
    pub base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub downstream_path_template: String,
    pub downstream_scheme: String,
    pub downstream_host_and_ports: Vec<DownstreamHostAndPort>,
    pub upstream_path_template: String,
    pub upstream_http_methods: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownstreamHostAndPort {
    pub host: String,
    pub port: i64,
}
