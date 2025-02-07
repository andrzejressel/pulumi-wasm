#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItem {
    #[builder(into)]
    #[serde(rename = "profileId")]
    pub r#profile_id: Box<String>,
    /// Query argument for field-level encryption query argument-profile mapping.
    #[builder(into)]
    #[serde(rename = "queryArg")]
    pub r#query_arg: Box<String>,
}
