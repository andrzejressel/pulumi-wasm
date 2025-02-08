#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkteamWorkerAccessConfigurationS3Presign {
    /// Use this parameter to specify the allowed request source. Possible sources are either SourceIp or VpcSourceIp. see IAM Policy Constraints details below.
    #[builder(into, default)]
    #[serde(rename = "iamPolicyConstraints")]
    pub r#iam_policy_constraints: Box<Option<super::super::types::sagemaker::WorkteamWorkerAccessConfigurationS3PresignIamPolicyConstraints>>,
}
