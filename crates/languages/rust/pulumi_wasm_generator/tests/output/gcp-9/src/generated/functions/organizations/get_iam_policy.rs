pub mod get_iam_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIamPolicyArgs {
        /// A nested configuration block that defines logging additional configuration for your project. This field is only supported on `gcp.projects.IAMPolicy`, `gcp.folder.IAMPolicy` and `gcp.organizations.IAMPolicy`.
        #[builder(into, default)]
        pub audit_configs: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::organizations::GetIamPolicyAuditConfig>,
            >,
        >,
        /// A nested configuration block (described below)
        /// defining a binding to be included in the policy document. Multiple
        /// `binding` arguments are supported.
        ///
        /// Each document configuration must have one or more `binding` blocks, which
        /// each accept the following arguments:
        #[builder(into, default)]
        pub bindings: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::organizations::GetIamPolicyBinding>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIamPolicyResult {
        pub audit_configs: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::organizations::GetIamPolicyAuditConfig>,
            >,
        >,
        pub bindings: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::organizations::GetIamPolicyBinding>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The above bindings serialized in a format suitable for
        /// referencing from a resource that supports IAM.
        pub policy_data: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetIamPolicyArgs,
    ) -> GetIamPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audit_configs_binding = args.audit_configs.get_output(context).get_inner();
        let bindings_binding = args.bindings.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getIAMPolicy:getIAMPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditConfigs".into(),
                    value: &audit_configs_binding,
                },
                register_interface::ObjectField {
                    name: "bindings".into(),
                    value: &bindings_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIamPolicyResult {
            audit_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("auditConfigs"),
            ),
            bindings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bindings"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            policy_data: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyData"),
            ),
        }
    }
}
