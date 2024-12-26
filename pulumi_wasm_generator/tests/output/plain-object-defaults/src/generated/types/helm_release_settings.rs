#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HelmReleaseSettings {
    /// The backend storage driver for Helm. Values are: configmap, secret, memory, sql.
    #[builder(into, default)]
    #[serde(rename = "driver")]
    pub r#driver: Box<Option<String>>,
    /// The path to the helm plugins directory.
    #[builder(into, default)]
    #[serde(rename = "pluginsPath")]
    pub r#plugins_path: Box<Option<String>>,
    /// to test required args
    #[builder(into)]
    #[serde(rename = "requiredArg")]
    pub r#required_arg: Box<String>,
}
