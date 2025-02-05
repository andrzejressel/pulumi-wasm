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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// The definition of the policy. See Definition below.
        #[builder(into, default)]
        pub definition: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::verifiedpermissions::PolicyDefinition>,
        >,
        /// The Policy Store ID of the policy store.
        #[builder(into)]
        pub policy_store_id: pulumi_wasm_rust::InputOrOutput<String>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_output(context).get_inner();
        let policy_store_id_binding = args
            .policy_store_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("definition"),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            policy_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyStoreId"),
            ),
        }
    }
}
