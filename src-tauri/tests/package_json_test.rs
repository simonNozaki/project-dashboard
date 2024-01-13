#[cfg(test)]
mod package_json_test {
    use package_dashboard::foundations::package_json::to_project_meta;

    #[test]
    fn can_parse_empty_project() {
        let raw = "{}".to_string();
        let (name, scripts) = to_project_meta(raw);

        assert_eq!(name, "無題のプロジェクト");
        assert!(scripts.is_empty());
    }

    #[test]
    fn can_parse_project() {
        let raw = "{ \"name\": \"test\", \"scripts\": { \"build\": \"yarn build\" } }".to_string();
        let (name, scripts) = to_project_meta(raw);

        assert_eq!(name, "test");
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts.get("build"), Some(&"yarn build".to_string()));
    }
}
