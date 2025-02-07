#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetProfilesProfilesProfile {
    /// ARN of the Profile.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// ID of the Profile.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Name of the Profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Share status of the Profile. Valid values [AWS docs](https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53profiles_Profile.html)
    #[builder(into)]
    #[serde(rename = "shareStatus")]
    pub r#share_status: Box<String>,
}
