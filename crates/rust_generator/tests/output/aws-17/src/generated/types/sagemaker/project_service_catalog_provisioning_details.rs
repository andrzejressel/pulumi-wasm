#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectServiceCatalogProvisioningDetails {
    /// The path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path.
    #[builder(into, default)]
    #[serde(rename = "pathId")]
    pub r#path_id: Box<Option<String>>,
    /// The ID of the product to provision.
    #[builder(into)]
    #[serde(rename = "productId")]
    pub r#product_id: Box<String>,
    /// The ID of the provisioning artifact.
    #[builder(into, default)]
    #[serde(rename = "provisioningArtifactId")]
    pub r#provisioning_artifact_id: Box<Option<String>>,
    /// A list of key value pairs that you specify when you provision a product. See Provisioning Parameter below.
    #[builder(into, default)]
    #[serde(rename = "provisioningParameters")]
    pub r#provisioning_parameters: Box<Option<Vec<super::super::types::sagemaker::ProjectServiceCatalogProvisioningDetailsProvisioningParameter>>>,
}
