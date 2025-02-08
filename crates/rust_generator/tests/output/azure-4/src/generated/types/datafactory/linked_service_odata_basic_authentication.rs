#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct LinkedServiceOdataBasicAuthentication {
    /// The password associated with the username, which can be used to authenticate to the OData endpoint.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The username which can be used to authenticate to the OData endpoint.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
