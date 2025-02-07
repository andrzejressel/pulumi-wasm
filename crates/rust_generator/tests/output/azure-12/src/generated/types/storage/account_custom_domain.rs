#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountCustomDomain {
    /// The Custom Domain Name to use for the Storage Account, which will be validated by Azure.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Should the Custom Domain Name be validated by using indirect CNAME validation?
    #[builder(into, default)]
    #[serde(rename = "useSubdomain")]
    pub r#use_subdomain: Box<Option<bool>>,
}
