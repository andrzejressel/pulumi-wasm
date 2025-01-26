pub mod get_organization_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyArgs {
        /// (Required) The name of the Constraint the Policy is configuring, for example, `serviceuser.services`. Check out the [complete list of available constraints](https://cloud.google.com/resource-manager/docs/organization-policy/understanding-constraints#available_constraints).
        #[builder(into)]
        pub constraint: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project ID.
        #[builder(into)]
        pub project: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationPolicyResult {
        pub boolean_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyBooleanPolicy>,
        >,
        pub constraint: pulumi_wasm_rust::Output<String>,
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub list_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyListPolicy>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        pub restore_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::projects::GetOrganizationPolicyRestorePolicy>,
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
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:projects/getOrganizationPolicy:getOrganizationPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "constraint".into(),
                    value: &constraint_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "booleanPolicies".into(),
                },
                register_interface::ResultField {
                    name: "constraint".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "listPolicies".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "restorePolicies".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrganizationPolicyResult {
            boolean_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("booleanPolicies").unwrap(),
            ),
            constraint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("constraint").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            list_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listPolicies").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            restore_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorePolicies").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
