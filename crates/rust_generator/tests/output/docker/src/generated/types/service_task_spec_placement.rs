#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceTaskSpecPlacement {
    /// An array of constraints. e.g.: `node.role==manager`
    #[builder(into, default)]
    #[serde(rename = "constraints")]
    pub r#constraints: Box<Option<Vec<String>>>,
    /// Maximum number of replicas for per node (default value is `0`, which is unlimited)
    #[builder(into, default)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    /// Platforms stores all the platforms that the service's image can run on
    #[builder(into, default)]
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Option<Vec<super::types::ServiceTaskSpecPlacementPlatform>>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`
    #[builder(into, default)]
    #[serde(rename = "prefs")]
    pub r#prefs: Box<Option<Vec<String>>>,
}
