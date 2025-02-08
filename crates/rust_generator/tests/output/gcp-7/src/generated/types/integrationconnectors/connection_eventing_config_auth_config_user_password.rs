#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionEventingConfigAuthConfigUserPassword {
    /// Password for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<super::super::types::integrationconnectors::ConnectionEventingConfigAuthConfigUserPasswordPassword>>,
    /// Username for Authentication.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
