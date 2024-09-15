#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecResources {
    /// Describes the resources which can be advertised by a node and requested by a task
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<crate::types::ServiceTaskSpecResourcesLimits>>,
    /// An object describing the resources which can be advertised by a node and requested by a task
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<crate::types::ServiceTaskSpecResourcesReservation>>,
}
