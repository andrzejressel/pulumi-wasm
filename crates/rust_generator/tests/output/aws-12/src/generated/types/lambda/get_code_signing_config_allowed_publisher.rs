#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCodeSigningConfigAllowedPublisher {
    /// The ARN for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.
    #[builder(into)]
    #[serde(rename = "signingProfileVersionArns")]
    pub r#signing_profile_version_arns: Box<Vec<String>>,
}
