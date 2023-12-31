use log::debug;
use reqwest::Response;

use crate::types::config::Route;

pub async fn request_downstream(route: &Route, path: &str) ->  Result<Response, Box<dyn std::error::Error>>{
    // Get first Downstream URL
    let downstream_host = &route.downstream_host_and_ports[0].host; 
    let downstream_port = &route.downstream_host_and_ports[0].port;

    // request downstream
    let request_uri = format!("{}://{}:{}{}", route.downstream_scheme, downstream_host, downstream_port, path);
    debug!("Requesting downstream: {}", &request_uri);

    // request content from request_uri
    let resp = reqwest::get(&request_uri)
    .await?;

    debug!("Status Code: {:#?}", resp.status());

    Ok(resp)
}