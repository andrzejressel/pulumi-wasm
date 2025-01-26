pub mod get_public_i_ps {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIPsArgs {
        /// The Allocation Type for the Public IP Address. Possible values include `Static` or `Dynamic`.
        #[builder(into, default)]
        pub allocation_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Filter to include IP Addresses which are attached to a device, such as a VM/LB (`Attached`) or unattached (`Unattached`).
        #[builder(into, default)]
        pub attachment_status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A prefix match used for the IP Addresses `name` field, case sensitive.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicIPsResult {
        pub allocation_type: pulumi_wasm_rust::Output<Option<String>>,
        pub attachment_status: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// A List of `public_ips` blocks as defined below filtered by the criteria above.
        pub public_ips: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetPublicIPsPublicIp>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPublicIPsArgs,
    ) -> GetPublicIPsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_type_binding = args
            .allocation_type
            .get_output(context)
            .get_inner();
        let attachment_status_binding = args
            .attachment_status
            .get_output(context)
            .get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getPublicIPs:getPublicIPs".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationType".into(),
                    value: &allocation_type_binding,
                },
                register_interface::ObjectField {
                    name: "attachmentStatus".into(),
                    value: &attachment_status_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocationType".into(),
                },
                register_interface::ResultField {
                    name: "attachmentStatus".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "publicIps".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPublicIPsResult {
            allocation_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationType").unwrap(),
            ),
            attachment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachmentStatus").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            public_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIps").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
