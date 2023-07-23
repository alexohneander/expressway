use crate::types::config::Route;

pub fn request_downstream(route: &Route){
    // Get first Downstream URL
    let downstream_host = &route.downstream_host_and_ports[0].host; 
    let downstream_port = &route.downstream_host_and_ports[0].port;

    // request downstream
    let request_uri = format!("{}://{}:{}{}", route.downstream_scheme, downstream_host, downstream_port, route.downstream_path_template);
    println!("Requesting downstream: {}", request_uri);
}