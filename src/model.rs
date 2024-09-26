use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StartSpringIoModel {
    links: Link,
    dependencies: Dependency,
    project_type: Project,
    packaging: Packaging,
    java_version: JavaVersion,
    language: Language,
    boot_version: BootVersion,
    group_id: Option<String>,
    artifact_id: Option<String>,
    version: Option<String>,
    name: String,
    description: String,
    package_name: String,
}

#[derive(Debug, Deserialize)]
struct Link {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Packaging {
    content: String,
}

#[derive(Debug, Deserialize)]
struct JavaVersion {
    content: String,
}

#[derive(Debug, Deserialize)]
struct Language {
    content: String,
}

#[derive(Debug, Deserialize)]
struct BootVersion {
    content: String,
}
