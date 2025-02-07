#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskAgentSetting {
    /// The number of cores required for the Container Registry Task. Possible value is `2`.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<i32>,
}
