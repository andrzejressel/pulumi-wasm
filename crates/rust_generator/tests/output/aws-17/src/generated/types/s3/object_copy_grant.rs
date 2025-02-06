#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ObjectCopyGrant {
    /// Email address of the grantee. Used only when `type` is `AmazonCustomerByEmail`.
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// Canonical user ID of the grantee. Used only when `type` is `CanonicalUser`.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// List of permissions to grant to grantee. Valid values are `READ`, `READ_ACP`, `WRITE_ACP`, `FULL_CONTROL`.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Vec<String>>,
    /// Type of grantee. Valid values are `CanonicalUser`, `Group`, and `AmazonCustomerByEmail`.
    /// 
    /// This configuration block has the following optional arguments (one of the three is required):
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// URI of the grantee group. Used only when `type` is `Group`.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
