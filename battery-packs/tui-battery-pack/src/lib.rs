#![doc = include_str!(concat!(env!("OUT_DIR"), "/docs.md"))]

#[cfg(test)]
mod tests {
    use ::battery_pack::testing::{PreviewBuilder, PreviewFile};
    use snapbox::{assert_data_eq, file};

    // Render the complete starter project into a stable, reviewable snapshot.
    fn render() -> String {
        let files: Vec<PreviewFile> = PreviewBuilder::new(env!("CARGO_MANIFEST_DIR"))
            .template("templates/app")
            .project_name("test-tui")
            .preview()
            .unwrap();
        let mut output = String::new();
        for file in files {
            output.push_str(&format!("── {} ──\n{}\n\n", file.path, file.content));
        }
        output
    }

    // Verify pack metadata and template structure.
    #[test]
    fn validate() {
        ::battery_pack::testing::validate(env!("CARGO_MANIFEST_DIR")).unwrap();
    }

    // Keep generated files and dependency declarations visible in review.
    #[test]
    fn snapshot_app_template() {
        assert_data_eq!(render(), file!["snapshots/app.txt"]);
    }
}
