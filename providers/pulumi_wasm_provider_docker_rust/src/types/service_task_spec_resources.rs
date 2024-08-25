#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecResources {
    /// Describes the resources which can be advertised by a node and requested by a task
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<crate::types::ServiceTaskSpecResourcesLimits>>,
    /// An object describing the resources which can be advertised by a node and requested by a task
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<crate::types::ServiceTaskSpecResourcesReservation>>,
}
