#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsAccountBodyScanning {
    /// Body scanning inspection mode. Available values: `deep`, `shallow`.
    #[builder(into)]
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}
