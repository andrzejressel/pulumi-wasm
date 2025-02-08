#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppIngressIpSecurityRestriction {
    /// The IP-filter action.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Description of the IP restriction rule that is being sent to the container-app.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// CIDR notation that matches the incoming IP address.
    #[builder(into)]
    #[serde(rename = "ipAddressRange")]
    pub r#ip_address_range: Box<String>,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
