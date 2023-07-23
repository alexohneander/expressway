#[cfg(test)]
mod tests {
    use expressway::configuration::configuration_loader;

    #[test]
    fn test_load_config() {
        let config = configuration_loader::load("config.json");
        assert_eq!(config.global_configuration.base_url, "localhost:4000");
    }
}