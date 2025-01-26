pub mod get_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// Domain name of the B2C tenant, including the `.onmicrosoft.com` suffix.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the AAD B2C Directory exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDirectoryResult {
        /// The type of billing for the AAD B2C tenant. Possible values include: `MAU` or `Auths`.
        pub billing_type: pulumi_wasm_rust::Output<String>,
        /// Location in which the B2C tenant is hosted and data resides. See [official docs](https://aka.ms/B2CDataResidenc) for more information.
        pub data_residency_location: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The date from which the billing type took effect. May not be populated until after the first billing cycle.
        pub effective_start_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Billing SKU for the B2C tenant. See [official docs](https://aka.ms/b2cBilling) for more information.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the AAD B2C Directory.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Tenant ID for the AAD B2C tenant.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDirectoryArgs,
    ) -> GetDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:aadb2c/getDirectory:getDirectory".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectoryResult {
            billing_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("billingType"),
            ),
            data_residency_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataResidencyLocation"),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            effective_start_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveStartDate"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
        }
    }
}
