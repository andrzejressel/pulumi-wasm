pub mod get_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyArgs {
        /// The unique identifier (ID) of the policy that you want more details on. Policy id starts with a "p-" followed by 8-28 lowercase or uppercase letters, digits, and underscores.
        #[builder(into)]
        pub policy_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyResult {
        /// The Amazon Resource Name of the policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Indicates if a policy is an AWS managed policy.
        pub aws_managed: pulumi_wasm_rust::Output<bool>,
        /// The text content of the policy.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The description of the policy.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The friendly name of the policy.
        pub name: pulumi_wasm_rust::Output<String>,
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The type of policy values can be `AISERVICES_OPT_OUT_POLICY | BACKUP_POLICY | RESOURCE_CONTROL_POLICY | SERVICE_CONTROL_POLICY | TAG_POLICY`
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicyArgs,
    ) -> GetPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_id_binding = args.policy_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getPolicy:getPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_managed: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsManaged"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
