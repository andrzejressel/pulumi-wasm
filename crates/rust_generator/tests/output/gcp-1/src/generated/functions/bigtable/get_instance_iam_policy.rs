pub mod get_instance_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceIamPolicyArgs {
        /// The name or relative resource id of the instance to manage IAM policies for.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance: pulumi_wasm_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceIamPolicyArgs,
    ) -> GetInstanceIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:bigtable/getInstanceIamPolicy:getInstanceIamPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceIamPolicyResult {
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
