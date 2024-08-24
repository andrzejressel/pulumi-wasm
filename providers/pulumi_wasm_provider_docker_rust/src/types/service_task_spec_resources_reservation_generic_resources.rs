#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    /// The Integer resources
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Box<Option<Vec<String>>>,
    /// The String resources
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Box<Option<Vec<String>>>,
}
