/// Provides a resource to create an AWS Firewall Manager policy. You need to be using AWS organizations and have enabled the Firewall Manager administrator account.
///
/// > **NOTE:** Due to limitations with testing, we provide it as best effort. If you find it useful, and have the ability to help test or notice issues, consider reaching out to us on GitHub.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:fms:Policy
///     properties:
///       name: FMS-Policy-Example
///       excludeResourceTags: false
///       remediationEnabled: false
///       resourceType: AWS::ElasticLoadBalancingV2::LoadBalancer
///       securityServicePolicyData:
///         type: WAF
///         managedServiceData:
///           fn::toJSON:
///             type: WAF
///             ruleGroups:
///               - id: ${exampleRuleGroup.id}
///                 overrideAction:
///                   type: COUNT
///             defaultAction:
///               type: BLOCK
///             overrideCustomerWebACLAssociation: false
///       tags:
///         Name: example-fms-policy
///   exampleRuleGroup:
///     type: aws:wafregional:RuleGroup
///     name: example
///     properties:
///       metricName: WAFRuleGroupExample
///       name: WAF-Rule-Group-Example
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Firewall Manager policies using the policy ID. For example:
///
/// ```sh
/// $ pulumi import aws:fms/policy:Policy example 5be49585-a7e3-4c49-dde1-a179fe4a619a
/// ```
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// If true, the request will also perform a clean-up process. Defaults to `true`. More information can be found here [AWS Firewall Manager delete policy](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_DeletePolicy.html)
        #[builder(into, default)]
        pub delete_all_policy_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, Firewall Manager will automatically remove protections from resources that leave the policy scope. Defaults to `false`. More information can be found here [AWS Firewall Manager policy contents](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html)
        #[builder(into, default)]
        pub delete_unused_fm_managed_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// The description of the AWS Network Firewall firewall policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of lists of accounts and OU's to exclude from the policy.
        #[builder(into, default)]
        pub exclude_map: pulumi_wasm_rust::Output<
            Option<super::super::types::fms::PolicyExcludeMap>,
        >,
        /// A boolean value, if true the tags that are specified in the `resource_tags` are not protected by this policy. If set to false and resource_tags are populated, resources that contain tags will be protected by this policy.
        #[builder(into)]
        pub exclude_resource_tags: pulumi_wasm_rust::Output<bool>,
        /// A map of lists of accounts and OU's to include in the policy.
        #[builder(into, default)]
        pub include_map: pulumi_wasm_rust::Output<
            Option<super::super::types::fms::PolicyIncludeMap>,
        >,
        /// The friendly name of the AWS Firewall Manager Policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A boolean value, indicates if the policy should automatically applied to resources that already exist in the account.
        #[builder(into, default)]
        pub remediation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub resource_set_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of resource tags, that if present will filter protections on resources based on the exclude_resource_tags.
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A resource type to protect. Conflicts with `resource_type_list`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values.
        #[builder(into, default)]
        pub resource_type: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of resource types to protect. Conflicts with `resource_type`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values. Lists with only one element are not supported, instead use `resource_type`.
        #[builder(into, default)]
        pub resource_type_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The objects to include in Security Service Policy Data. Documented below.
        #[builder(into)]
        pub security_service_policy_data: pulumi_wasm_rust::Output<
            super::super::types::fms::PolicySecurityServicePolicyData,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// If true, the request will also perform a clean-up process. Defaults to `true`. More information can be found here [AWS Firewall Manager delete policy](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_DeletePolicy.html)
        pub delete_all_policy_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, Firewall Manager will automatically remove protections from resources that leave the policy scope. Defaults to `false`. More information can be found here [AWS Firewall Manager policy contents](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html)
        pub delete_unused_fm_managed_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// The description of the AWS Network Firewall firewall policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of lists of accounts and OU's to exclude from the policy.
        pub exclude_map: pulumi_wasm_rust::Output<
            Option<super::super::types::fms::PolicyExcludeMap>,
        >,
        /// A boolean value, if true the tags that are specified in the `resource_tags` are not protected by this policy. If set to false and resource_tags are populated, resources that contain tags will be protected by this policy.
        pub exclude_resource_tags: pulumi_wasm_rust::Output<bool>,
        /// A map of lists of accounts and OU's to include in the policy.
        pub include_map: pulumi_wasm_rust::Output<
            Option<super::super::types::fms::PolicyIncludeMap>,
        >,
        /// The friendly name of the AWS Firewall Manager Policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for each update to the policy.
        pub policy_update_token: pulumi_wasm_rust::Output<String>,
        /// A boolean value, indicates if the policy should automatically applied to resources that already exist in the account.
        pub remediation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub resource_set_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of resource tags, that if present will filter protections on resources based on the exclude_resource_tags.
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A resource type to protect. Conflicts with `resource_type_list`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// A list of resource types to protect. Conflicts with `resource_type`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values. Lists with only one element are not supported, instead use `resource_type`.
        pub resource_type_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The objects to include in Security Service Policy Data. Documented below.
        pub security_service_policy_data: pulumi_wasm_rust::Output<
            super::super::types::fms::PolicySecurityServicePolicyData,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delete_all_policy_resources_binding = args
            .delete_all_policy_resources
            .get_inner();
        let delete_unused_fm_managed_resources_binding = args
            .delete_unused_fm_managed_resources
            .get_inner();
        let description_binding = args.description.get_inner();
        let exclude_map_binding = args.exclude_map.get_inner();
        let exclude_resource_tags_binding = args.exclude_resource_tags.get_inner();
        let include_map_binding = args.include_map.get_inner();
        let name_binding = args.name.get_inner();
        let remediation_enabled_binding = args.remediation_enabled.get_inner();
        let resource_set_ids_binding = args.resource_set_ids.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let resource_type_lists_binding = args.resource_type_lists.get_inner();
        let security_service_policy_data_binding = args
            .security_service_policy_data
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:fms/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deleteAllPolicyResources".into(),
                    value: &delete_all_policy_resources_binding,
                },
                register_interface::ObjectField {
                    name: "deleteUnusedFmManagedResources".into(),
                    value: &delete_unused_fm_managed_resources_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "excludeMap".into(),
                    value: &exclude_map_binding,
                },
                register_interface::ObjectField {
                    name: "excludeResourceTags".into(),
                    value: &exclude_resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "includeMap".into(),
                    value: &include_map_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "remediationEnabled".into(),
                    value: &remediation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSetIds".into(),
                    value: &resource_set_ids_binding,
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
                    name: "resourceTypeLists".into(),
                    value: &resource_type_lists_binding,
                },
                register_interface::ObjectField {
                    name: "securityServicePolicyData".into(),
                    value: &security_service_policy_data_binding,
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
                    name: "deleteAllPolicyResources".into(),
                },
                register_interface::ResultField {
                    name: "deleteUnusedFmManagedResources".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "excludeMap".into(),
                },
                register_interface::ResultField {
                    name: "excludeResourceTags".into(),
                },
                register_interface::ResultField {
                    name: "includeMap".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyUpdateToken".into(),
                },
                register_interface::ResultField {
                    name: "remediationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceSetIds".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypeLists".into(),
                },
                register_interface::ResultField {
                    name: "securityServicePolicyData".into(),
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
        PolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delete_all_policy_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteAllPolicyResources").unwrap(),
            ),
            delete_unused_fm_managed_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteUnusedFmManagedResources").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            exclude_map: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeMap").unwrap(),
            ),
            exclude_resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeResourceTags").unwrap(),
            ),
            include_map: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeMap").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_update_token: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyUpdateToken").unwrap(),
            ),
            remediation_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remediationEnabled").unwrap(),
            ),
            resource_set_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSetIds").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            resource_type_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypeLists").unwrap(),
            ),
            security_service_policy_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityServicePolicyData").unwrap(),
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
