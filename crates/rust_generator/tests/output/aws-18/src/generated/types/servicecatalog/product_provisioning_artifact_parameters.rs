#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProductProvisioningArtifactParameters {
    /// Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Whether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
    #[builder(into, default)]
    #[serde(rename = "disableTemplateValidation")]
    pub r#disable_template_validation: Box<Option<bool>>,
    /// Name of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Template source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
    #[builder(into, default)]
    #[serde(rename = "templatePhysicalId")]
    pub r#template_physical_id: Box<Option<String>>,
    /// Template source as URL of the CloudFormation template in Amazon S3.
    #[builder(into, default)]
    #[serde(rename = "templateUrl")]
    pub r#template_url: Box<Option<String>>,
    /// Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
