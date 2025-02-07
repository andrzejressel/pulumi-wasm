#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerHost {
    /// Hostname to add
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// IP address this hostname should resolve to.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
