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
pub mod provisioned_product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisionedProductArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// _Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
        #[builder(into, default)]
        pub ignore_errors: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-friendly name of the provisioned product.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
        #[builder(into, default)]
        pub notification_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
        #[builder(into, default)]
        pub path_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the path. You must provide `path_id` or `path_name`, but not both.
        #[builder(into, default)]
        pub path_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Product identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
        #[builder(into, default)]
        pub product_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the product. You must provide `product_id` or `product_name`, but not both.
        #[builder(into, default)]
        pub product_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        #[builder(into, default)]
        pub provisioning_artifact_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        #[builder(into, default)]
        pub provisioning_artifact_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
        #[builder(into, default)]
        pub provisioning_parameters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::servicecatalog::ProvisionedProductProvisioningParameter,
                >,
            >,
        >,
        /// _Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
        #[builder(into, default)]
        pub retain_physical_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
        #[builder(into, default)]
        pub stack_set_provisioning_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::servicecatalog::ProvisionedProductStackSetProvisioningPreferences,
            >,
        >,
        /// Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProvisionedProductResult {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the provisioned product.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Set of CloudWatch dashboards that were created when provisioning the product.
        pub cloudwatch_dashboard_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Time when the provisioned product was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// _Only applies to deleting._ If set to `true`, AWS Service Catalog stops managing the specified provisioned product even if it cannot delete the underlying resources. The default value is `false`.
        pub ignore_errors: pulumi_wasm_rust::Output<Option<bool>>,
        /// Record identifier of the last request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
        pub last_provisioning_record_id: pulumi_wasm_rust::Output<String>,
        /// Record identifier of the last request performed on this provisioned product.
        pub last_record_id: pulumi_wasm_rust::Output<String>,
        /// Record identifier of the last successful request performed on this provisioned product of the following types: `ProvisionedProduct`, `UpdateProvisionedProduct`, `ExecuteProvisionedProductPlan`, `TerminateProvisionedProduct`.
        pub last_successful_provisioning_record_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the launch role associated with the provisioned product.
        pub launch_role_arn: pulumi_wasm_rust::Output<String>,
        /// User-friendly name of the provisioned product.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Passed to CloudFormation. The SNS topic ARNs to which to publish stack-related events.
        pub notification_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The set of outputs for the product created.
        pub outputs: pulumi_wasm_rust::Output<
            Vec<super::super::types::servicecatalog::ProvisionedProductOutput>,
        >,
        /// Path identifier of the product. This value is optional if the product has a default path, and required if the product has more than one path. To list the paths for a product, use `aws.servicecatalog.getLaunchPaths`. When required, you must provide `path_id` or `path_name`, but not both.
        pub path_id: pulumi_wasm_rust::Output<String>,
        /// Name of the path. You must provide `path_id` or `path_name`, but not both.
        pub path_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Product identifier. For example, `prod-abcdzk7xy33qa`. You must provide `product_id` or `product_name`, but not both.
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// Name of the product. You must provide `product_id` or `product_name`, but not both.
        pub product_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of the provisioning artifact. For example, `pa-4abcdjnxjj6ne`. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        pub provisioning_artifact_id: pulumi_wasm_rust::Output<String>,
        /// Name of the provisioning artifact. You must provide the `provisioning_artifact_id` or `provisioning_artifact_name`, but not both.
        pub provisioning_artifact_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with parameters specified by the administrator that are required for provisioning the product. See `provisioning_parameters` Block for details.
        pub provisioning_parameters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::servicecatalog::ProvisionedProductProvisioningParameter,
                >,
            >,
        >,
        /// _Only applies to deleting._ Whether to delete the Service Catalog provisioned product but leave the CloudFormation stack, stack set, or the underlying resources of the deleted provisioned product. The default value is `false`.
        pub retain_physical_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block with information about the provisioning preferences for a stack set. See `stack_set_provisioning_preferences` Block for details.
        pub stack_set_provisioning_preferences: pulumi_wasm_rust::Output<
            Option<
                super::super::types::servicecatalog::ProvisionedProductStackSetProvisioningPreferences,
            >,
        >,
        /// Current status of the provisioned product. See meanings below.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Current status message of the provisioned product.
        pub status_message: pulumi_wasm_rust::Output<String>,
        /// Tags to apply to the provisioned product. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of provisioned product. Valid values are `CFN_STACK` and `CFN_STACKSET`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProvisionedProductArgs) -> ProvisionedProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let ignore_errors_binding = args.ignore_errors.get_inner();
        let name_binding = args.name.get_inner();
        let notification_arns_binding = args.notification_arns.get_inner();
        let path_id_binding = args.path_id.get_inner();
        let path_name_binding = args.path_name.get_inner();
        let product_id_binding = args.product_id.get_inner();
        let product_name_binding = args.product_name.get_inner();
        let provisioning_artifact_id_binding = args.provisioning_artifact_id.get_inner();
        let provisioning_artifact_name_binding = args
            .provisioning_artifact_name
            .get_inner();
        let provisioning_parameters_binding = args.provisioning_parameters.get_inner();
        let retain_physical_resources_binding = args
            .retain_physical_resources
            .get_inner();
        let stack_set_provisioning_preferences_binding = args
            .stack_set_provisioning_preferences
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/provisionedProduct:ProvisionedProduct".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreErrors".into(),
                    value: &ignore_errors_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationArns".into(),
                    value: &notification_arns_binding,
                },
                register_interface::ObjectField {
                    name: "pathId".into(),
                    value: &path_id_binding,
                },
                register_interface::ObjectField {
                    name: "pathName".into(),
                    value: &path_name_binding,
                },
                register_interface::ObjectField {
                    name: "productId".into(),
                    value: &product_id_binding,
                },
                register_interface::ObjectField {
                    name: "productName".into(),
                    value: &product_name_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningArtifactId".into(),
                    value: &provisioning_artifact_id_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningArtifactName".into(),
                    value: &provisioning_artifact_name_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningParameters".into(),
                    value: &provisioning_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "retainPhysicalResources".into(),
                    value: &retain_physical_resources_binding,
                },
                register_interface::ObjectField {
                    name: "stackSetProvisioningPreferences".into(),
                    value: &stack_set_provisioning_preferences_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cloudwatchDashboardNames".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "ignoreErrors".into(),
                },
                register_interface::ResultField {
                    name: "lastProvisioningRecordId".into(),
                },
                register_interface::ResultField {
                    name: "lastRecordId".into(),
                },
                register_interface::ResultField {
                    name: "lastSuccessfulProvisioningRecordId".into(),
                },
                register_interface::ResultField {
                    name: "launchRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationArns".into(),
                },
                register_interface::ResultField {
                    name: "outputs".into(),
                },
                register_interface::ResultField {
                    name: "pathId".into(),
                },
                register_interface::ResultField {
                    name: "pathName".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "productName".into(),
                },
                register_interface::ResultField {
                    name: "provisioningArtifactId".into(),
                },
                register_interface::ResultField {
                    name: "provisioningArtifactName".into(),
                },
                register_interface::ResultField {
                    name: "provisioningParameters".into(),
                },
                register_interface::ResultField {
                    name: "retainPhysicalResources".into(),
                },
                register_interface::ResultField {
                    name: "stackSetProvisioningPreferences".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProvisionedProductResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cloudwatch_dashboard_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudwatchDashboardNames").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            ignore_errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreErrors").unwrap(),
            ),
            last_provisioning_record_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastProvisioningRecordId").unwrap(),
            ),
            last_record_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastRecordId").unwrap(),
            ),
            last_successful_provisioning_record_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastSuccessfulProvisioningRecordId").unwrap(),
            ),
            launch_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("launchRoleArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationArns").unwrap(),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputs").unwrap(),
            ),
            path_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathId").unwrap(),
            ),
            path_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathName").unwrap(),
            ),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            product_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productName").unwrap(),
            ),
            provisioning_artifact_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningArtifactId").unwrap(),
            ),
            provisioning_artifact_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningArtifactName").unwrap(),
            ),
            provisioning_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningParameters").unwrap(),
            ),
            retain_physical_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainPhysicalResources").unwrap(),
            ),
            stack_set_provisioning_preferences: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackSetProvisioningPreferences").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
