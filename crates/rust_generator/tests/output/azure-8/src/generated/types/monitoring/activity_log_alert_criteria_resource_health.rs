#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ActivityLogAlertCriteriaResourceHealth {
    /// The current resource health statuses that will log an alert. Possible values are `Available`, `Degraded`, `Unavailable` and `Unknown`.
    #[builder(into, default)]
    #[serde(rename = "currents")]
    pub r#currents: Box<Option<Vec<String>>>,
    /// The previous resource health statuses that will log an alert. Possible values are `Available`, `Degraded`, `Unavailable` and `Unknown`.
    #[builder(into, default)]
    #[serde(rename = "previouses")]
    pub r#previouses: Box<Option<Vec<String>>>,
    /// The reason that will log an alert. Possible values are `PlatformInitiated` (such as a problem with the resource in an affected region of an Azure incident), `UserInitiated` (such as a shutdown request of a VM) and `Unknown`.
    #[builder(into, default)]
    #[serde(rename = "reasons")]
    pub r#reasons: Box<Option<Vec<String>>>,
}
