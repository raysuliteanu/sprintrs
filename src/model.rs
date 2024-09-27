use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSpringIoModel {
    #[serde(rename(deserialize = "_links"), skip)]
    links: Link,
    dependencies: Dependency,
    #[serde(rename(deserialize = "type"), skip)]
    project_type: Project,
    packaging: Packaging,
    java_version: JavaVersion,
    language: Language,
    boot_version: BootVersion,
    group_id: Option<TypeAndDefault>,
    artifact_id: Option<TypeAndDefault>,
    version: Option<TypeAndDefault>,
    name: Option<TypeAndDefault>,
    description: Option<TypeAndDefault>,
    package_name: Option<TypeAndDefault>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Link {}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Dependency {
    #[serde(rename(deserialize = "type"))]
    widget_type: String,
    values: Vec<DependencyValue>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct DependencyValue {
    id: String,
    name: String,
    description: String,
    #[serde(rename(deserialize = "_links"), skip)]
    links: Link,
    version_range: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Project {
    #[serde(rename(deserialize = "type"))]
    action_type: String,
    default: String,
    values: Vec<ProjectType>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct ProjectType {
    id: String,
    name: String,
    description: String,
    action: Option<String>,
    tags: Option<Vec<(String, String)>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Packaging {
    #[serde(rename(deserialize = "type"))]
    action_type: String,
    default: String,
    values: Vec<IdAndName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct JavaVersion {
    #[serde(rename(deserialize = "type"))]
    widget_type: String,
    default: String,
    values: Vec<IdAndName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct Language {
    #[serde(rename(deserialize = "type"))]
    widget_type: String,
    default: String,
    values: Vec<IdAndName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct BootVersion {
    #[serde(rename(deserialize = "type"))]
    widget_type: String,
    default: String,
    values: Vec<IdAndName>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct IdAndName {
    id: String,
    name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct TypeAndDefault {
    #[serde(rename(deserialize = "type"))]
    default_type: String,
    #[serde(rename(deserialize = "default"))]
    default_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_trivial() {
        let json = r#"
{
    "_links": {},
    "dependencies": {},
    "type": {},
    "packaging": {},
    "javaVersion": {},
    "language": {},
    "bootVersion": {}
}
        "#;

        let model = serde_json::from_str::<StartSpringIoModel>(json);
        if model.is_err() {
            println!("{:#?}", model);
        }
        assert!(model.is_ok());
    }

    #[test]
    fn deserialize_full() {
        let json = include_str!("../start.spring.io.json");
        let model = serde_json::from_str::<StartSpringIoModel>(json);
        assert!(model.is_ok());
    }
}
