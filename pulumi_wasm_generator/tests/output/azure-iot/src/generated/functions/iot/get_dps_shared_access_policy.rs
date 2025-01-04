pub mod get_dps_shared_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDpsSharedAccessPolicyArgs {
        /// Specifies the name of the IoT Hub Device Provisioning service to which the Shared Access Policy belongs.
        #[builder(into)]
        pub iothub_dps_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the IotHub Shared Access Policy.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group under which the IotHub Shared Access Policy resource exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDpsSharedAccessPolicyResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub iothub_dps_name: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Shared Access Policy.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary key used to create the authentication token.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Shared Access Policy.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary key used to create the authentication token.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDpsSharedAccessPolicyArgs) -> GetDpsSharedAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let iothub_dps_name_binding = args.iothub_dps_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:iot/getDpsSharedAccessPolicy:getDpsSharedAccessPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "iothubDpsName".into(),
                    value: &iothub_dps_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "iothubDpsName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "secondaryKey".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDpsSharedAccessPolicyResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            iothub_dps_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubDpsName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryKey").unwrap(),
            ),
        }
    }
}
