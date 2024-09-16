#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    /// The Integer resources
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Box<Option<Vec<String>>>,
    /// The String resources
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Box<Option<Vec<String>>>,
}
