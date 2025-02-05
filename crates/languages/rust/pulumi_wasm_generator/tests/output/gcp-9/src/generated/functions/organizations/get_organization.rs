pub mod get_organization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationArgs {
        /// The domain name of the Organization.
        ///
        /// > **NOTE:** One of `organization` or `domain` must be specified.
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Organization's numeric ID, including an optional `organizations/` prefix.
        #[builder(into, default)]
        pub organization: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationResult {
        /// Timestamp when the Organization was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The Google for Work customer ID of the Organization.
        pub directory_customer_id: pulumi_wasm_rust::Output<String>,
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Organization's current lifecycle state.
        pub lifecycle_state: pulumi_wasm_rust::Output<String>,
        /// The resource name of the Organization in the form `organizations/{organization_id}`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Organization ID.
        pub org_id: pulumi_wasm_rust::Output<String>,
        pub organization: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrganizationArgs,
    ) -> GetOrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_output(context).get_inner();
        let organization_binding = args.organization.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getOrganization:getOrganization".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "organization".into(),
                    value: &organization_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrganizationResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            directory_customer_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryCustomerId"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            lifecycle_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifecycleState"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            organization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organization"),
            ),
        }
    }
}
