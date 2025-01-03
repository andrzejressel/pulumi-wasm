/// Resource for managing an AWS Verified Permissions Policy.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:verifiedpermissions:Policy
///     properties:
///       policyStoreId: ${testAwsVerifiedpermissionsPolicyStore.id}
///       definition:
///         static:
///           statement: 'permit (principal, action == Action::"view", resource in Album:: "test_album");'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy using the `policy_id,policy_store_id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedpermissions/policy:Policy example policy-id-12345678,policy-store-id-12345678
/// ```
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// The definition of the policy. See Definition below.
        #[builder(into, default)]
        pub definition: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedpermissions::PolicyDefinition>,
        >,
        /// The Policy Store ID of the policy store.
        #[builder(into)]
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// The date the policy was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The definition of the policy. See Definition below.
        pub definition: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedpermissions::PolicyDefinition>,
        >,
        /// The Policy ID of the policy.
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The Policy Store ID of the policy store.
        pub policy_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PolicyArgs) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_inner();
        let policy_store_id_binding = args.policy_store_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policy:Policy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "policyStoreId".into(),
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
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyStoreId").unwrap(),
            ),
        }
    }
}
