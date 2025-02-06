pub mod get_scheduling_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSchedulingPolicyArgs {
        /// ARN of the scheduling policy.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSchedulingPolicyResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub fair_share_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::batch::GetSchedulingPolicyFairSharePolicy>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the scheduling policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSchedulingPolicyArgs,
    ) -> GetSchedulingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:batch/getSchedulingPolicy:getSchedulingPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSchedulingPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            fair_share_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fairSharePolicies"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
