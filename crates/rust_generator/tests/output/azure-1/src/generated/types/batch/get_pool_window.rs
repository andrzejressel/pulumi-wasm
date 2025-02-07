#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolWindow {
    /// Whether automatic updates are enabled on the virtual machine.
    #[builder(into)]
    #[serde(rename = "enableAutomaticUpdates")]
    pub r#enable_automatic_updates: Box<bool>,
}
