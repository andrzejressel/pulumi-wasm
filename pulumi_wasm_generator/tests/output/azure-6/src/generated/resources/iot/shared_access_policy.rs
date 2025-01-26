/// Manages an IotHub Shared Access Policy
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleSharedAccessPolicy:
///     type: azure:iot:SharedAccessPolicy
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       registryRead: true
///       registryWrite: true
/// ```
///
/// ## Import
///
/// IoTHub Shared Access Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/sharedAccessPolicy:SharedAccessPolicy shared_access_policy1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/iotHubKeys/shared_access_policy1
/// ```
///
pub mod shared_access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedAccessPolicyArgs {
        /// Adds `DeviceConnect` permission to this Shared Access Account. It allows sending and receiving on the device-side endpoints.
        ///
        /// > **NOTE** At least one of `registry_read`, `registry_write`, `service_connect`, `device_connect` permissions must be set to `true`.
        #[builder(into, default)]
        pub device_connect: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the IoTHub to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Adds `RegistryRead` permission to this Shared Access Account. It allows read access to the identity registry.
        #[builder(into, default)]
        pub registry_read: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Adds `RegistryWrite` permission to this Shared Access Account. It allows write access to the identity registry.
        ///
        /// > **NOTE** When `registry_write` is set to `true`, `registry_read` must also be set to true. This is a limitation of the Azure REST API
        #[builder(into, default)]
        pub registry_write: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Adds `ServiceConnect` permission to this Shared Access Account. It allows sending and receiving on the cloud-side endpoints.
        #[builder(into, default)]
        pub service_connect: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SharedAccessPolicyResult {
        /// Adds `DeviceConnect` permission to this Shared Access Account. It allows sending and receiving on the device-side endpoints.
        ///
        /// > **NOTE** At least one of `registry_read`, `registry_write`, `service_connect`, `device_connect` permissions must be set to `true`.
        pub device_connect: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the IoTHub to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Shared Access Policy.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary key used to create the authentication token.
        pub primary_key: pulumi_wasm_rust::Output<String>,
        /// Adds `RegistryRead` permission to this Shared Access Account. It allows read access to the identity registry.
        pub registry_read: pulumi_wasm_rust::Output<Option<bool>>,
        /// Adds `RegistryWrite` permission to this Shared Access Account. It allows write access to the identity registry.
        ///
        /// > **NOTE** When `registry_write` is set to `true`, `registry_read` must also be set to true. This is a limitation of the Azure REST API
        pub registry_write: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Shared Access Policy.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary key used to create the authentication token.
        pub secondary_key: pulumi_wasm_rust::Output<String>,
        /// Adds `ServiceConnect` permission to this Shared Access Account. It allows sending and receiving on the cloud-side endpoints.
        pub service_connect: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SharedAccessPolicyArgs,
    ) -> SharedAccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_connect_binding = args.device_connect.get_output(context).get_inner();
        let iothub_name_binding = args.iothub_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let registry_read_binding = args.registry_read.get_output(context).get_inner();
        let registry_write_binding = args.registry_write.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let service_connect_binding = args
            .service_connect
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/sharedAccessPolicy:SharedAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceConnect".into(),
                    value: &device_connect_binding,
                },
                register_interface::ObjectField {
                    name: "iothubName".into(),
                    value: &iothub_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "registryRead".into(),
                    value: &registry_read_binding,
                },
                register_interface::ObjectField {
                    name: "registryWrite".into(),
                    value: &registry_write_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceConnect".into(),
                    value: &service_connect_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deviceConnect".into(),
                },
                register_interface::ResultField {
                    name: "iothubName".into(),
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
                    name: "registryRead".into(),
                },
                register_interface::ResultField {
                    name: "registryWrite".into(),
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
                register_interface::ResultField {
                    name: "serviceConnect".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SharedAccessPolicyResult {
            device_connect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceConnect").unwrap(),
            ),
            iothub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubName").unwrap(),
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
            registry_read: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryRead").unwrap(),
            ),
            registry_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryWrite").unwrap(),
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
            service_connect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceConnect").unwrap(),
            ),
        }
    }
}
