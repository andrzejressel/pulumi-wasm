#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FieldLevelEncryptionConfigQueryArgProfileConfig {
    /// Flag to set if you want a request to be forwarded to the origin even if the profile specified by the field-level encryption query argument, fle-profile, is unknown.
    #[builder(into)]
    #[serde(rename = "forwardWhenQueryArgProfileIsUnknown")]
    pub r#forward_when_query_arg_profile_is_unknown: Box<bool>,
    /// Object that contains an attribute `items` that contains the list ofrofiles specified for query argument-profile mapping for field-level encryption. see Query Arg Profile.
    #[builder(into, default)]
    #[serde(rename = "queryArgProfiles")]
    pub r#query_arg_profiles: Box<Option<super::super::types::cloudfront::FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles>>,
}
