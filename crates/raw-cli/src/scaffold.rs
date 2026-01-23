// Credit: Ben Ajaero

use std::fs;
use std::path::Path;

pub fn create_project(name: &str) -> Result<(), String> {
    create_project_at(Path::new("."), name)
}

fn create_project_at(root: &Path, name: &str) -> Result<(), String> {
    let project_root = root.join(name);
    if project_root.exists() {
        return Err(format!("directory already exists: {}", project_root.display()));
    }

    fs::create_dir_all(project_root.join("src")).map_err(map_io)?;
    fs::create_dir_all(project_root.join("public")).map_err(map_io)?;

    fs::write(project_root.join("Cargo.toml"), cargo_toml(name)).map_err(map_io)?;
    fs::write(project_root.join("src/main.rs"), main_rs()).map_err(map_io)?;
    fs::write(project_root.join("README.md"), readme(name)).map_err(map_io)?;
    fs::write(project_root.join("public/index.html"), index_html(name)).map_err(map_io)?;
    fs::write(project_root.join(".gitignore"), gitignore()).map_err(map_io)?;

    Ok(())
}

fn map_io(err: std::io::Error) -> String {
    err.to_string()
}

fn cargo_toml(name: &str) -> String {
    format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nraw = \"0.1\"\ntokio = {{ version = \"1\", features = [\"rt-multi-thread\", \"macros\"] }}\n",
        name
    )
}

fn main_rs() -> &'static str {
    "// Credit: Ben Ajaero\n\nuse raw::{App, Text};\n\n#[tokio::main]\nasync fn main() {\n    let mut app = App::new();\n    app.get(\"/\", |_req| async { Text::new(\"Hello from Raw\") });\n    app.listen(\"127.0.0.1:3000\").await.unwrap();\n}\n"
}

fn readme(name: &str) -> String {
    format!(
        "# {}\n\nGenerated with Raw CLI.\n\n## Run\n```bash\ncargo run\n```\n",
        name
    )
}

fn index_html(_name: &str) -> &'static str {
    "<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Raw App</title>\n  </head>\n  <body>\n    <main>\n      <h1>Raw App</h1>\n      <p>Powered by Raw.</p>\n    </main>\n  </body>\n</html>\n"
}

fn gitignore() -> &'static str {
    "/target\nCargo.lock\n"
}

#[cfg(test)]
mod tests {
    use super::create_project_at;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn scaffold_creates_project_structure() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("timestamp")
            .as_millis();
        let temp_root = std::env::temp_dir().join(format!("raw-cli-test-{}", timestamp));
        fs::create_dir_all(&temp_root).expect("create temp root");

        let result = create_project_at(&temp_root, "example");
        assert!(result.is_ok());

        let project_root = temp_root.join("example");
        assert!(project_root.join("Cargo.toml").exists());
        assert!(project_root.join("src/main.rs").exists());
        assert!(project_root.join("public/index.html").exists());

        let _ = fs::remove_dir_all(&temp_root);
    }
}
