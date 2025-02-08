#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KubernetesClusterExtensionPlan {
    /// Specifies the name of the plan from the marketplace. Changing this forces a new Kubernetes Cluster Extension to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the product of the plan from the marketplace. Changing this forces a new Kubernetes Cluster Extension to be created.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// Specifies the promotion code to use with the plan. Changing this forces a new Kubernetes Cluster Extension to be created.
    #[builder(into, default)]
    #[serde(rename = "promotionCode")]
    pub r#promotion_code: Box<Option<String>>,
    /// Specifies the publisher of the plan. Changing this forces a new Kubernetes Cluster Extension to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// Specifies the version of the plan from the marketplace. Changing this forces a new Kubernetes Cluster Extension to be created.
    /// 
    /// > **NOTE:** When `plan` is specified, legal terms must be accepted for this item on this subscription before creating the Kubernetes Cluster Extension. The `azure.marketplace.Agreement` resource or AZ CLI tool can be used to do this.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
