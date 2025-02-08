#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AddonPodIdentityAssociation {
    /// The Amazon Resource Name (ARN) of the IAM role to associate with the service account. The EKS Pod Identity agent manages credentials to assume this role for applications in the containers in the pods that use this service account.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The name of the Kubernetes service account inside the cluster to associate the IAM credentials with.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
}
