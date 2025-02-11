/// Manages a Service Catalog Provisioning Artifact for a specified product.
///
/// > A "provisioning artifact" is also referred to as a "version."
///
/// > **NOTE:** You cannot create a provisioning artifact for a product that was shared with you.
///
/// > **NOTE:** The user or role that use this resource must have the `cloudformation:GetTemplate` IAM policy permission. This policy permission is required when using the `template_physical_id` argument.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = provisioning_artifact::create(
///         "example",
///         ProvisioningArtifactArgs::builder()
///             .name("example")
///             .product_id("${exampleAwsServicecatalogProduct.id}")
///             .template_url(
///                 "https://${exampleAwsS3Bucket.bucketRegionalDomainName}/${exampleAwsS3Object.key}",
///             )
///             .type_("CLOUD_FORMATION_TEMPLATE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_provisioning_artifact` using the provisioning artifact ID and product ID separated by a colon. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/provisioningArtifact:ProvisioningArtifact example pa-ij2b6lusy6dec:prod-el3an0rma3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod provisioning_artifact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisioningArtifactArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the product version is active. Inactive provisioning artifacts are invisible to end users. End users cannot launch or update a provisioned product from an inactive provisioning artifact. Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
        #[builder(into, default)]
        pub disable_template_validation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Information set by the administrator to provide guidance to end users about which provisioning artifacts to use. Valid values are `DEFAULT` and `DEPRECATED`. The default is `DEFAULT`. Users are able to make updates to a provisioned product of a deprecated version but cannot launch new provisioned products using a deprecated version.
        #[builder(into, default)]
        pub guidance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the product.
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Template source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
        #[builder(into, default)]
        pub template_physical_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Template source as URL of the CloudFormation template in Amazon S3.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub template_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProvisioningArtifactResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). The default value is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the product version is active. Inactive provisioning artifacts are invisible to end users. End users cannot launch or update a provisioned product from an inactive provisioning artifact. Default is `true`.
        pub active: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Time when the provisioning artifact was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the provisioning artifact (i.e., version), including how it differs from the previous provisioning artifact.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Whether AWS Service Catalog stops validating the specified provisioning artifact template even if it is invalid.
        pub disable_template_validation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Information set by the administrator to provide guidance to end users about which provisioning artifacts to use. Valid values are `DEFAULT` and `DEPRECATED`. The default is `DEFAULT`. Users are able to make updates to a provisioned product of a deprecated version but cannot launch new provisioned products using a deprecated version.
        pub guidance: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the provisioning artifact (for example, `v1`, `v2beta`). No spaces are allowed.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the product.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Provisioning artifact identifier.
        pub provisioning_artifact_id: pulumi_gestalt_rust::Output<String>,
        /// Template source as the physical ID of the resource that contains the template. Currently only supports CloudFormation stack ARN. Specify the physical ID as `arn:[partition]:cloudformation:[region]:[account ID]:stack/[stack name]/[resource ID]`.
        pub template_physical_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Template source as URL of the CloudFormation template in Amazon S3.
        ///
        /// The following arguments are optional:
        pub template_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of provisioning artifact. See [AWS Docs](https://docs.aws.amazon.com/servicecatalog/latest/dg/API_ProvisioningArtifactProperties.html) for valid list of values.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProvisioningArtifactArgs,
    ) -> ProvisioningArtifactResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let active_binding = args.active.get_output(context);
        let description_binding = args.description.get_output(context);
        let disable_template_validation_binding = args
            .disable_template_validation
            .get_output(context);
        let guidance_binding = args.guidance.get_output(context);
        let name_binding = args.name.get_output(context);
        let product_id_binding = args.product_id.get_output(context);
        let template_physical_id_binding = args.template_physical_id.get_output(context);
        let template_url_binding = args.template_url.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/provisioningArtifact:ProvisioningArtifact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "active".into(),
                    value: &active_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableTemplateValidation".into(),
                    value: &disable_template_validation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guidance".into(),
                    value: &guidance_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productId".into(),
                    value: &product_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templatePhysicalId".into(),
                    value: &template_physical_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateUrl".into(),
                    value: &template_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProvisioningArtifactResult {
            accept_language: o.get_field("acceptLanguage"),
            active: o.get_field("active"),
            created_time: o.get_field("createdTime"),
            description: o.get_field("description"),
            disable_template_validation: o.get_field("disableTemplateValidation"),
            guidance: o.get_field("guidance"),
            name: o.get_field("name"),
            product_id: o.get_field("productId"),
            provisioning_artifact_id: o.get_field("provisioningArtifactId"),
            template_physical_id: o.get_field("templatePhysicalId"),
            template_url: o.get_field("templateUrl"),
            type_: o.get_field("type"),
        }
    }
}
