#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPrincipalPolicySimulationContext {
    /// The context _condition key_ to set.
    /// 
    /// If you have policies containing `Condition` elements or using dynamic interpolations then you will need to provide suitable values for each condition key your policies use. See [Actions, resources, and condition keys for AWS services](https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html) to find the various condition keys that are normally provided for real requests to each action of each AWS service.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// An IAM value type that determines how the policy simulator will interpret the strings given in `values`.
    /// 
    /// For more information, see the `ContextKeyType` field of [`iam.ContextEntry`](https://docs.aws.amazon.com/IAM/latest/APIReference/API_ContextEntry.html) in the underlying API.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// A set of one or more values for this context entry.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
