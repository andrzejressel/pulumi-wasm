/// Resource for managing an AWS RBin Rule.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rbin:Rule
///     properties:
///       description: example_rule
///       resourceType: EBS_SNAPSHOT
///       resourceTags:
///         - resourceTagKey: tag_key
///           resourceTagValue: tag_value
///       retentionPeriod:
///         retentionPeriodValue: 10
///         retentionPeriodUnit: DAYS
///       tags:
///         test_tag_key: test_tag_value
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RBin Rule using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rbin/rule:Rule example examplerule
/// ```
pub mod rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// The retention rule description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Information about the retention rule lock configuration. See `lock_configuration` below.
        #[builder(into, default)]
        pub lock_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::rbin::RuleLockConfiguration>,
        >,
        /// Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::rbin::RuleResourceTag>>,
        >,
        /// The resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub retention_period: pulumi_wasm_rust::Output<
            super::super::types::rbin::RuleRetentionPeriod,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The retention rule description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Information about the retention rule lock configuration. See `lock_configuration` below.
        pub lock_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::rbin::RuleLockConfiguration>,
        >,
        /// (Timestamp) The date and time at which the unlock delay is set to expire. Only returned for retention rules that have been unlocked and that are still within the unlock delay period.
        pub lock_end_time: pulumi_wasm_rust::Output<String>,
        /// (Optional) The lock state of the retention rules to list. Only retention rules with the specified lock state are returned. Valid values are `locked`, `pending_unlock`, `unlocked`.
        pub lock_state: pulumi_wasm_rust::Output<String>,
        /// Specifies the resource tags to use to identify resources that are to be retained by a tag-level retention rule. See `resource_tags` below.
        pub resource_tags: pulumi_wasm_rust::Output<
            Vec<super::super::types::rbin::RuleResourceTag>,
        >,
        /// The resource type to be retained by the retention rule. Valid values are `EBS_SNAPSHOT` and `EC2_IMAGE`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Information about the retention period for which the retention rule is to retain resources. See `retention_period` below.
        ///
        /// The following arguments are optional:
        pub retention_period: pulumi_wasm_rust::Output<
            super::super::types::rbin::RuleRetentionPeriod,
        >,
        /// (String) The state of the retention rule. Only retention rules that are in the `available` state retain resources. Valid values include `pending` and `available`.
        pub status: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RuleArgs) -> RuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let lock_configuration_binding = args.lock_configuration.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let retention_period_binding = args.retention_period.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rbin/rule:Rule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "lockConfiguration".into(),
                    value: &lock_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPeriod".into(),
                    value: &retention_period_binding,
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
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lockConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "lockEndTime".into(),
                },
                register_interface::ResultField {
                    name: "lockState".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "retentionPeriod".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        RuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            lock_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockConfiguration").unwrap(),
            ),
            lock_end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockEndTime").unwrap(),
            ),
            lock_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lockState").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionPeriod").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
