#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIamPolicyArgs {
        /// A nested configuration block that defines logging additional configuration for your project. This field is only supported on `gcp.projects.IAMPolicy`, `gcp.folder.IAMPolicy` and `gcp.organizations.IAMPolicy`.
        #[builder(into, default)]
        pub audit_configs: pulumi_gestalt_rust::InputOrOutput<
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
        pub bindings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::organizations::GetIamPolicyBinding>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetIamPolicyResult {
        pub audit_configs: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::super::types::organizations::GetIamPolicyAuditConfig>,
            >,
        >,
        pub bindings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::organizations::GetIamPolicyBinding>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The above bindings serialized in a format suitable for
        /// referencing from a resource that supports IAM.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetIamPolicyArgs,
    ) -> GetIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audit_configs_binding = args.audit_configs.get_output(context);
        let bindings_binding = args.bindings.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getIAMPolicy:getIAMPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "auditConfigs".into(),
                    value: &audit_configs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bindings".into(),
                    value: &bindings_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIamPolicyResult {
            audit_configs: o.get_field("auditConfigs"),
            bindings: o.get_field("bindings"),
            id: o.get_field("id"),
            policy_data: o.get_field("policyData"),
        }
    }
}
