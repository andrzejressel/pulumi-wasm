#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCloudVmClusterPropertyTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
