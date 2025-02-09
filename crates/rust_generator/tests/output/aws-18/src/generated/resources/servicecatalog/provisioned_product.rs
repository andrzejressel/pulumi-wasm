/// This resource provisions and manages a Service Catalog provisioned product.
///
/// A provisioned product is a resourced instance of a product. For example, provisioning a product based on a CloudFormation template launches a CloudFormation stack and its underlying resources.
///
/// Like this resource, the `aws_servicecatalog_record` data source also provides information about a provisioned product. Although a Service Catalog record provides some overlapping information with this resource, a record is tied to a provisioned product event, such as provisioning, termination, and updating.
///
/// > **Tip:** If you include conflicted keys as tags, AWS will report an error, "Parameter validation failed: Missing required parameter in Tags[N]:Value".
///
/// > **Tip:** A "provisioning artifact" is also referred to as a "version." A "distributor" is also referred to as a "vendor."
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicecatalog:ProvisionedProduct
///     properties:
///       name: example
///       productName: Example product
///       provisioningArtifactName: Example version
///       provisioningParameters:
///         - key: foo
///           value: bar
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_provisioned_product` using the provisioned product ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/provisionedProduct:ProvisionedProduct example pp-dnigbtea24ste
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod provisioned_product {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisionedProductArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// _Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
        #[builder(into, default)]
        pub ignore_errors: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User-friendly name of the provisioned product.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
        #[builder(into, default)]
        pub notification_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
        #[builder(into, default)]
        pub path_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the path. You must provide `path_id` or `path_name`, but not both.
        #[builder(into, default)]
        pub path_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Product identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
        #[builder(into, default)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the product. You must provide `product_id` or `product_name`, but not both.
        #[builder(into, default)]
        pub product_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        #[builder(into, default)]
        pub provisioning_artifact_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        #[builder(into, default)]
        pub provisioning_artifact_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
        #[builder(into, default)]
        pub provisioning_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::servicecatalog::ProvisionedProductProvisioningParameter,
                >,
            >,
        >,
        /// _Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
        #[builder(into, default)]
        pub retain_physical_resources: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
        #[builder(into, default)]
        pub stack_set_provisioning_preferences: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::servicecatalog::ProvisionedProductStackSetProvisioningPreferences,
            >,
        >,
        /// Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProvisionedProductResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the provisioned product.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of CloudWatch dashboards that were created when provisioning the product.
        pub cloudwatch_dashboard_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Time when the provisioned product was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// _Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
        pub ignore_errors: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Record identifier of the last request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
        pub last_provisioning_record_id: pulumi_gestalt_rust::Output<String>,
        /// Record identifier of the last request performed on this provisioned product.
        pub last_record_id: pulumi_gestalt_rust::Output<String>,
        /// Record identifier of the last successful request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
        pub last_successful_provisioning_record_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the launch role associated with the provisioned product.
        pub launch_role_arn: pulumi_gestalt_rust::Output<String>,
        /// User-friendly name of the provisioned product.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
        pub notification_arns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The set of outputs for the product created.
        pub outputs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::servicecatalog::ProvisionedProductOutput>,
        >,
        /// Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
        pub path_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the path. You must provide `path_id` or `path_name`, but not both.
        pub path_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Product identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the product. You must provide `product_id` or `product_name`, but not both.
        pub product_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        pub provisioning_artifact_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        pub provisioning_artifact_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
        pub provisioning_parameters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::servicecatalog::ProvisionedProductProvisioningParameter,
                >,
            >,
        >,
        /// _Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
        pub retain_physical_resources: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
        pub stack_set_provisioning_preferences: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::servicecatalog::ProvisionedProductStackSetProvisioningPreferences,
            >,
        >,
        /// Current status of the provisioned product. See meanings below.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Current status message of the provisioned product.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of provisioned product. Valid values are `CFN_STACK` and `CFN_STACKSET`.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProvisionedProductArgs,
    ) -> ProvisionedProductResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_language_binding = args.accept_language.get_output(context);
        let ignore_errors_binding = args.ignore_errors.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_arns_binding = args.notification_arns.get_output(context);
        let path_id_binding = args.path_id.get_output(context);
        let path_name_binding = args.path_name.get_output(context);
        let product_id_binding = args.product_id.get_output(context);
        let product_name_binding = args.product_name.get_output(context);
        let provisioning_artifact_id_binding = args
            .provisioning_artifact_id
            .get_output(context);
        let provisioning_artifact_name_binding = args
            .provisioning_artifact_name
            .get_output(context);
        let provisioning_parameters_binding = args
            .provisioning_parameters
            .get_output(context);
        let retain_physical_resources_binding = args
            .retain_physical_resources
            .get_output(context);
        let stack_set_provisioning_preferences_binding = args
            .stack_set_provisioning_preferences
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicecatalog/provisionedProduct:ProvisionedProduct".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptLanguage".into(),
                    value: accept_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreErrors".into(),
                    value: ignore_errors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationArns".into(),
                    value: notification_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathId".into(),
                    value: path_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathName".into(),
                    value: path_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productId".into(),
                    value: product_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "productName".into(),
                    value: product_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisioningArtifactId".into(),
                    value: provisioning_artifact_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisioningArtifactName".into(),
                    value: provisioning_artifact_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "provisioningParameters".into(),
                    value: provisioning_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retainPhysicalResources".into(),
                    value: retain_physical_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackSetProvisioningPreferences".into(),
                    value: stack_set_provisioning_preferences_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProvisionedProductResult {
            accept_language: o.get_field("acceptLanguage"),
            arn: o.get_field("arn"),
            cloudwatch_dashboard_names: o.get_field("cloudwatchDashboardNames"),
            created_time: o.get_field("createdTime"),
            ignore_errors: o.get_field("ignoreErrors"),
            last_provisioning_record_id: o.get_field("lastProvisioningRecordId"),
            last_record_id: o.get_field("lastRecordId"),
            last_successful_provisioning_record_id: o
                .get_field("lastSuccessfulProvisioningRecordId"),
            launch_role_arn: o.get_field("launchRoleArn"),
            name: o.get_field("name"),
            notification_arns: o.get_field("notificationArns"),
            outputs: o.get_field("outputs"),
            path_id: o.get_field("pathId"),
            path_name: o.get_field("pathName"),
            product_id: o.get_field("productId"),
            product_name: o.get_field("productName"),
            provisioning_artifact_id: o.get_field("provisioningArtifactId"),
            provisioning_artifact_name: o.get_field("provisioningArtifactName"),
            provisioning_parameters: o.get_field("provisioningParameters"),
            retain_physical_resources: o.get_field("retainPhysicalResources"),
            stack_set_provisioning_preferences: o
                .get_field("stackSetProvisioningPreferences"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
