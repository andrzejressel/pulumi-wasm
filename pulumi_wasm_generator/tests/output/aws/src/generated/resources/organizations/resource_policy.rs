/// Provides a resource to manage a resource-based delegation policy that can be used to delegate policy management for AWS Organizations to specified member accounts to perform policy actions that are by default available only to the management account. See the [_AWS Organizations User Guide_](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_delegate_policies.html) for more information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_policy::create(
///         "example",
///         ResourcePolicyArgs::builder()
///             .content(
///                 "{\n  \"Version\": \"2012-10-17\",\n  \"Statement\": [\n    {\n      \"Sid\": \"DelegatingNecessaryDescribeListActions\",\n      \"Effect\": \"Allow\",\n      \"Principal\": {\n        \"AWS\": \"arn:aws:iam::123456789012:root\"\n      },\n      \"Action\": [\n        \"organizations:DescribeOrganization\",\n        \"organizations:DescribeOrganizationalUnit\",\n        \"organizations:DescribeAccount\",\n        \"organizations:DescribePolicy\",\n        \"organizations:DescribeEffectivePolicy\",\n        \"organizations:ListRoots\",\n        \"organizations:ListOrganizationalUnitsForParent\",\n        \"organizations:ListParents\",\n        \"organizations:ListChildren\",\n        \"organizations:ListAccounts\",\n        \"organizations:ListAccountsForParent\",\n        \"organizations:ListPolicies\",\n        \"organizations:ListPoliciesForTarget\",\n        \"organizations:ListTargetsForPolicy\",\n        \"organizations:ListTagsForResource\"\n      ],\n      \"Resource\": \"*\"\n    }\n  ]\n}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_organizations_resource_policy` using the resource policy ID. For example:
///
/// ```sh
/// $ pulumi import aws:organizations/resourcePolicy:ResourcePolicy example rp-12345678
/// ```
pub mod resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// Content for the resource policy. The text must be correctly formatted JSON that complies with the syntax for the resource policy's type. See the [_AWS Organizations User Guide_](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_delegate_examples.html) for examples.
        #[builder(into)]
        pub content: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// Amazon Resource Name (ARN) of the resource policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Content for the resource policy. The text must be correctly formatted JSON that complies with the syntax for the resource policy's type. See the [_AWS Organizations User Guide_](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_delegate_examples.html) for examples.
        pub content: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: ResourcePolicyArgs) -> ResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:organizations/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
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
                    name: "content".into(),
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
        ResourcePolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("content").unwrap(),
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