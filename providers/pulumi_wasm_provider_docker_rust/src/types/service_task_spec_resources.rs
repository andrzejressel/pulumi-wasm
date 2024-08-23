#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResources {
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<crate::types::ServiceTaskSpecResourcesLimits>>,
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<crate::types::ServiceTaskSpecResourcesReservation>>,
}
