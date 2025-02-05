pub mod get_organizational_unit_child_accounts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitChildAccountsArgs {
        /// The parent ID of the accounts.
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationalUnitChildAccountsResult {
        /// List of child accounts, which have the following attributes:
        pub accounts: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::organizations::GetOrganizationalUnitChildAccountsAccount,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parent_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrganizationalUnitChildAccountsArgs,
    ) -> GetOrganizationalUnitChildAccountsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getOrganizationalUnitChildAccounts:getOrganizationalUnitChildAccounts"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrganizationalUnitChildAccountsResult {
            accounts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accounts"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parentId"),
            ),
        }
    }
}
