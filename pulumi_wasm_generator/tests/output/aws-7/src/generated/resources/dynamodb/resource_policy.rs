/// Resource for managing an AWS DynamoDB Resource Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_policy::create(
///         "example",
///         ResourcePolicyArgs::builder()
///             .policy("${test.json}")
///             .resource_arn("${exampleAwsDynamodbTable.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB Resource Policy using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/resourcePolicy:ResourcePolicy example arn:aws:dynamodb:us-east-1:1234567890:table/my-table
/// ```
pub mod resource_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourcePolicyArgs {
        /// Set this parameter to true to confirm that you want to remove your permissions to change the policy of this resource in the future.
        #[builder(into, default)]
        pub confirm_remove_self_resource_access: pulumi_wasm_rust::Output<Option<bool>>,
        /// n Amazon Web Services resource-based policy document in JSON format. The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit. For a full list of all considerations that you should keep in mind while attaching a resource-based policy, see Resource-based policy considerations.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams. You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the Resource field of a given Statement in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple Statement fields in your policy document.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourcePolicyResult {
        /// Set this parameter to true to confirm that you want to remove your permissions to change the policy of this resource in the future.
        pub confirm_remove_self_resource_access: pulumi_wasm_rust::Output<bool>,
        /// n Amazon Web Services resource-based policy document in JSON format. The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit. For a full list of all considerations that you should keep in mind while attaching a resource-based policy, see Resource-based policy considerations.
        ///
        /// The following arguments are optional:
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams. You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the Resource field of a given Statement in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple Statement fields in your policy document.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// A unique string that represents the revision ID of the policy. If you are comparing revision IDs, make sure to always use string comparison logic.
        pub revision_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourcePolicyArgs) -> ResourcePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let confirm_remove_self_resource_access_binding = args
            .confirm_remove_self_resource_access
            .get_inner();
        let policy_binding = args.policy.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/resourcePolicy:ResourcePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "confirmRemoveSelfResourceAccess".into(),
                    value: &confirm_remove_self_resource_access_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "confirmRemoveSelfResourceAccess".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "revisionId".into(),
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
            confirm_remove_self_resource_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confirmRemoveSelfResourceAccess").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            revision_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revisionId").unwrap(),
            ),
        }
    }
}
