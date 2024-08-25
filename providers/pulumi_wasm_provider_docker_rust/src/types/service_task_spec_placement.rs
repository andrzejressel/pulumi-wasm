#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecPlacement {
    /// An array of constraints. e.g.: `node.role==manager`
    #[serde(rename = "constraints")]
    pub r#constraints: Box<Option<Vec<String>>>,
    /// Maximum number of replicas for per node (default value is `0`, which is unlimited)
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    /// Platforms stores all the platforms that the service's image can run on
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`
    #[serde(rename = "prefs")]
    pub r#prefs: Box<Option<Vec<String>>>,
}
