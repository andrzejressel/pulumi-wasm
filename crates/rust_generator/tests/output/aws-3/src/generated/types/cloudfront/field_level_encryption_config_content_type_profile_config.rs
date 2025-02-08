#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FieldLevelEncryptionConfigContentTypeProfileConfig {
    /// Object that contains an attribute `items` that contains the list of configurations for a field-level encryption content type-profile. See Content Type Profile.
    #[builder(into)]
    #[serde(rename = "contentTypeProfiles")]
    pub r#content_type_profiles: Box<super::super::types::cloudfront::FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles>,
    /// specifies what to do when an unknown content type is provided for the profile. If true, content is forwarded without being encrypted when the content type is unknown. If false (the default), an error is returned when the content type is unknown.
    #[builder(into)]
    #[serde(rename = "forwardWhenContentTypeIsUnknown")]
    pub r#forward_when_content_type_is_unknown: Box<bool>,
}
