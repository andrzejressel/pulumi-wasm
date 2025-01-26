pub mod get_organization_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyArgs {
        /// (Required) The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        #[builder(into)]
        pub constraint: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the folder to set the policy for. Its format is folders/{folder_id}.
        #[builder(into)]
        pub folder: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyResult {
        pub boolean_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::folder::GetOrganizationPolicyBooleanPolicy>,
        >,
        pub constraint: pulumi_wasm_rust::Output<String>,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub folder: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub list_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::folder::GetOrganizationPolicyListPolicy>,
        >,
        pub restore_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::folder::GetOrganizationPolicyRestorePolicy>,
        >,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrganizationPolicyArgs,
    ) -> GetOrganizationPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let constraint_binding = args.constraint.get_output(context).get_inner();
        let folder_binding = args.folder.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:folder/getOrganizationPolicy:getOrganizationPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "constraint".into(),
                    value: &constraint_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrganizationPolicyResult {
            boolean_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("booleanPolicies"),
            ),
            constraint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("constraint"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            folder: pulumi_wasm_rust::__private::into_domain(o.extract_field("folder")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            list_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("listPolicies"),
            ),
            restore_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restorePolicies"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
