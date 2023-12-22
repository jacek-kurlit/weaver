use weaver_core::components::files::load_content::LoadContent;

#[test]
fn should_load_content_from_file() {
    let load_content = LoadContent {
        file_path: get_resource_path("file1.txt"),
    };
    let content = load_content.execute().unwrap();
    assert_eq!(content, "File 1 content\n");
}

#[test]
fn should_return_error_when_file_not_exists() {
    let load_content = LoadContent {
        file_path: get_resource_path("unknown_file.txt"),
    };
    let result = load_content.execute();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        //TODO: we should have our own error types
        "No such file or directory (os error 2)"
    );
}

fn get_resource_path(resouce_name: &str) -> String {
    format!(
        "{}/tests/resources/{}",
        env!("CARGO_MANIFEST_DIR"),
        resouce_name
    )
}
