#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryTaskRegistryCredential {
    /// One or more `custom` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "customs")]
    pub r#customs: Box<Option<Vec<super::super::types::containerservice::RegistryTaskRegistryCredentialCustom>>>,
    /// One `source` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<super::super::types::containerservice::RegistryTaskRegistryCredentialSource>>,
}
