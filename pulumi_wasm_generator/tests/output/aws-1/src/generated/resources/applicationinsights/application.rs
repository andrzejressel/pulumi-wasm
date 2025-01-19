/// Provides a ApplicationInsights Application resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:applicationinsights:Application
///     properties:
///       resourceGroupName: ${exampleGroup.name}
///   exampleGroup:
///     type: aws:resourcegroups:Group
///     name: example
///     properties:
///       name: example
///       resourceQuery:
///         query:
///           fn::toJSON:
///             ResourceTypeFilters:
///               - AWS::EC2::Instance
///             TagFilters:
///               - Key: Stage
///                 Values:
///                   - Test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ApplicationInsights Applications using the `resource_group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:applicationinsights/application:Application some some-application
/// ```
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// Indicates whether Application Insights automatically configures unmonitored resources in the resource group.
        #[builder(into, default)]
        pub auto_config_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configures all of the resources in the resource group by applying the recommended configurations.
        #[builder(into, default)]
        pub auto_create: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
        #[builder(into, default)]
        pub cwe_monitor_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
        #[builder(into, default)]
        pub grouping_type: pulumi_wasm_rust::Output<Option<String>>,
        /// When set to `true`, creates opsItems for any problems detected on an application.
        #[builder(into, default)]
        pub ops_center_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
        #[builder(into, default)]
        pub ops_item_sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource group.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// ARN of the Application.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Indicates whether Application Insights automatically configures unmonitored resources in the resource group.
        pub auto_config_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configures all of the resources in the resource group by applying the recommended configurations.
        pub auto_create: pulumi_wasm_rust::Output<Option<bool>>,
        /// Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as instance terminated, failed deployment, and others.
        pub cwe_monitor_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Application Insights can create applications based on a resource group or on an account. To create an account-based application using all of the resources in the account, set this parameter to `ACCOUNT_BASED`.
        pub grouping_type: pulumi_wasm_rust::Output<Option<String>>,
        /// When set to `true`, creates opsItems for any problems detected on an application.
        pub ops_center_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.
        pub ops_item_sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource group.
        ///
        /// The following arguments are optional:
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_config_enabled_binding = args.auto_config_enabled.get_inner();
        let auto_create_binding = args.auto_create.get_inner();
        let cwe_monitor_enabled_binding = args.cwe_monitor_enabled.get_inner();
        let grouping_type_binding = args.grouping_type.get_inner();
        let ops_center_enabled_binding = args.ops_center_enabled.get_inner();
        let ops_item_sns_topic_arn_binding = args.ops_item_sns_topic_arn.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:applicationinsights/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoConfigEnabled".into(),
                    value: &auto_config_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "autoCreate".into(),
                    value: &auto_create_binding,
                },
                register_interface::ObjectField {
                    name: "cweMonitorEnabled".into(),
                    value: &cwe_monitor_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "groupingType".into(),
                    value: &grouping_type_binding,
                },
                register_interface::ObjectField {
                    name: "opsCenterEnabled".into(),
                    value: &ops_center_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "opsItemSnsTopicArn".into(),
                    value: &ops_item_sns_topic_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoConfigEnabled".into(),
                },
                register_interface::ResultField {
                    name: "autoCreate".into(),
                },
                register_interface::ResultField {
                    name: "cweMonitorEnabled".into(),
                },
                register_interface::ResultField {
                    name: "groupingType".into(),
                },
                register_interface::ResultField {
                    name: "opsCenterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "opsItemSnsTopicArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_config_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoConfigEnabled").unwrap(),
            ),
            auto_create: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoCreate").unwrap(),
            ),
            cwe_monitor_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cweMonitorEnabled").unwrap(),
            ),
            grouping_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupingType").unwrap(),
            ),
            ops_center_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opsCenterEnabled").unwrap(),
            ),
            ops_item_sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("opsItemSnsTopicArn").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
