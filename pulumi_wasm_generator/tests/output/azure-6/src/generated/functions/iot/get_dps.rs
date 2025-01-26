pub mod get_dps {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDpsArgs {
        /// Specifies the name of the Iot Device Provisioning Service resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group under which the Iot Device Provisioning Service is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDpsResult {
        /// The allocation policy of the IoT Device Provisioning Service.
        pub allocation_policy: pulumi_wasm_rust::Output<String>,
        /// The device endpoint of the IoT Device Provisioning Service.
        pub device_provisioning_host_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the IoT Device Provisioning Service.
        pub id_scope: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the IoT Device Provisioning Service exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The service endpoint of the IoT Device Provisioning Service.
        pub service_operations_host_name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDpsArgs,
    ) -> GetDpsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:iot/getDps:getDps".into(),
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
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDpsResult {
            allocation_policy: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allocationPolicy"),
            ),
            device_provisioning_host_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceProvisioningHostName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            id_scope: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("idScope"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            service_operations_host_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceOperationsHostName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
