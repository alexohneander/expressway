// BEGIN: 6f7c3f8d7b5d
#[cfg(test)]
mod tests {
    use expressway::configuration::configuration_loader;

    #[test]
    fn test_load_config() {
        let config = configuration_loader::load("config.json");
        assert_eq!(config.global_configuration.base_url, "localhost:4000");
    }

    // #[test]
    // fn test_load_invalid_config() {
       
    //     let result = configuration_loader::load("invalid_config.json");
    //     assert!(result.is_err());
    // }
}
// END: 6f7c3f8d7b5d