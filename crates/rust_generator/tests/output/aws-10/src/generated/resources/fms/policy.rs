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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// If true, the request will also perform a clean-up process. Defaults to `true`. More information can be found here [AWS Firewall Manager delete policy](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_DeletePolicy.html)
        #[builder(into, default)]
        pub delete_all_policy_resources: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// If true, Firewall Manager will automatically remove protections from resources that leave the policy scope. Defaults to `false`. More information can be found here [AWS Firewall Manager policy contents](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html)
        #[builder(into, default)]
        pub delete_unused_fm_managed_resources: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The description of the AWS Network Firewall firewall policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of lists of accounts and OU's to exclude from the policy.
        #[builder(into, default)]
        pub exclude_map: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fms::PolicyExcludeMap>,
        >,
        /// A boolean value, if true the tags that are specified in the `resource_tags` are not protected by this policy. If set to false and resource_tags are populated, resources that contain tags will be protected by this policy.
        #[builder(into)]
        pub exclude_resource_tags: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// A map of lists of accounts and OU's to include in the policy.
        #[builder(into, default)]
        pub include_map: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::fms::PolicyIncludeMap>,
        >,
        /// The friendly name of the AWS Firewall Manager Policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A boolean value, indicates if the policy should automatically applied to resources that already exist in the account.
        #[builder(into, default)]
        pub remediation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub resource_set_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of resource tags, that if present will filter protections on resources based on the exclude_resource_tags.
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A resource type to protect. Conflicts with `resource_type_list`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values.
        #[builder(into, default)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of resource types to protect. Conflicts with `resource_type`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values. Lists with only one element are not supported, instead use `resource_type`.
        #[builder(into, default)]
        pub resource_type_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The objects to include in Security Service Policy Data. Documented below.
        #[builder(into)]
        pub security_service_policy_data: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::fms::PolicySecurityServicePolicyData,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// If true, the request will also perform a clean-up process. Defaults to `true`. More information can be found here [AWS Firewall Manager delete policy](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_DeletePolicy.html)
        pub delete_all_policy_resources: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If true, Firewall Manager will automatically remove protections from resources that leave the policy scope. Defaults to `false`. More information can be found here [AWS Firewall Manager policy contents](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html)
        pub delete_unused_fm_managed_resources: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The description of the AWS Network Firewall firewall policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of lists of accounts and OU's to exclude from the policy.
        pub exclude_map: pulumi_gestalt_rust::Output<
            Option<super::super::types::fms::PolicyExcludeMap>,
        >,
        /// A boolean value, if true the tags that are specified in the `resource_tags` are not protected by this policy. If set to false and resource_tags are populated, resources that contain tags will be protected by this policy.
        pub exclude_resource_tags: pulumi_gestalt_rust::Output<bool>,
        /// A map of lists of accounts and OU's to include in the policy.
        pub include_map: pulumi_gestalt_rust::Output<
            Option<super::super::types::fms::PolicyIncludeMap>,
        >,
        /// The friendly name of the AWS Firewall Manager Policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for each update to the policy.
        pub policy_update_token: pulumi_gestalt_rust::Output<String>,
        /// A boolean value, indicates if the policy should automatically applied to resources that already exist in the account.
        pub remediation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub resource_set_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of resource tags, that if present will filter protections on resources based on the exclude_resource_tags.
        pub resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A resource type to protect. Conflicts with `resource_type_list`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// A list of resource types to protect. Conflicts with `resource_type`. See the [FMS API Reference](https://docs.aws.amazon.com/fms/2018-01-01/APIReference/API_Policy.html#fms-Type-Policy-ResourceType) for more information about supported values. Lists with only one element are not supported, instead use `resource_type`.
        pub resource_type_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The objects to include in Security Service Policy Data. Documented below.
        pub security_service_policy_data: pulumi_gestalt_rust::Output<
            super::super::types::fms::PolicySecurityServicePolicyData,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let delete_all_policy_resources_binding = args
            .delete_all_policy_resources
            .get_output(context)
            .get_inner();
        let delete_unused_fm_managed_resources_binding = args
            .delete_unused_fm_managed_resources
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let exclude_map_binding = args.exclude_map.get_output(context).get_inner();
        let exclude_resource_tags_binding = args
            .exclude_resource_tags
            .get_output(context)
            .get_inner();
        let include_map_binding = args.include_map.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let remediation_enabled_binding = args
            .remediation_enabled
            .get_output(context)
            .get_inner();
        let resource_set_ids_binding = args
            .resource_set_ids
            .get_output(context)
            .get_inner();
        let resource_tags_binding = args.resource_tags.get_output(context).get_inner();
        let resource_type_binding = args.resource_type.get_output(context).get_inner();
        let resource_type_lists_binding = args
            .resource_type_lists
            .get_output(context)
            .get_inner();
        let security_service_policy_data_binding = args
            .security_service_policy_data
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            delete_all_policy_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteAllPolicyResources"),
            ),
            delete_unused_fm_managed_resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteUnusedFmManagedResources"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            exclude_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeMap"),
            ),
            exclude_resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeResourceTags"),
            ),
            include_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includeMap"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy_update_token: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyUpdateToken"),
            ),
            remediation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remediationEnabled"),
            ),
            resource_set_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceSetIds"),
            ),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
            resource_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceType"),
            ),
            resource_type_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTypeLists"),
            ),
            security_service_policy_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityServicePolicyData"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
