#[cfg(test)]
mod tests {
    use expressway::{types::config::{Route, DownstreamHostAndPort}, services::reverse_proxy::request_downstream};


    #[test]
    fn test_request_downstream() {
        let route = Route {
            downstream_scheme: "http".to_string(),
            downstream_host_and_ports: vec![DownstreamHostAndPort {
                host: "localhost".to_string(),
                port: 8080,
            }],
            downstream_path_template: "/api/users".to_string(),
            upstream_path_template: "/api/users".to_string(),
            upstream_http_methods: vec!["get".to_string()],
        };

        request_downstream(&route);

        // Add more assertions here if needed
    }
}