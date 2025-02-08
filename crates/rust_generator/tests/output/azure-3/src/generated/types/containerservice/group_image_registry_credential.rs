#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GroupImageRegistryCredential {
    /// The password with which to connect to the registry. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The address to use to connect to the registry without protocol ("https"/"http"). For example: "myacr.acr.io". Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
    /// The identity ID for the private registry. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
    /// The username with which to connect to the registry. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
