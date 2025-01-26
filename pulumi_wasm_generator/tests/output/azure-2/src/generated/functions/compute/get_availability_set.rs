pub mod get_availability_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAvailabilitySetArgs {
        /// The name of the Availability Set.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which the Availability Set exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAvailabilitySetResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The supported Azure location where the Availability Set exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Whether the availability set is managed or not.
        pub managed: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of fault domains that are used.
        pub platform_fault_domain_count: pulumi_wasm_rust::Output<String>,
        /// The number of update domains that are used.
        pub platform_update_domain_count: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAvailabilitySetArgs,
    ) -> GetAvailabilitySetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getAvailabilitySet:getAvailabilitySet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAvailabilitySetResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            managed: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managed"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            platform_fault_domain_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformFaultDomainCount"),
            ),
            platform_update_domain_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformUpdateDomainCount"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
