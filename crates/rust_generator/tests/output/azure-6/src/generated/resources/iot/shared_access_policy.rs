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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedAccessPolicyArgs {
        /// Adds `DeviceConnect` permission to this Shared Access Account. It allows sending and receiving on the device-side endpoints.
        ///
        /// > **NOTE** At least one of `registry_read`, `registry_write`, `service_connect`, `device_connect` permissions must be set to `true`.
        #[builder(into, default)]
        pub device_connect: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the IoTHub to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Adds `RegistryRead` permission to this Shared Access Account. It allows read access to the identity registry.
        #[builder(into, default)]
        pub registry_read: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Adds `RegistryWrite` permission to this Shared Access Account. It allows write access to the identity registry.
        ///
        /// > **NOTE** When `registry_write` is set to `true`, `registry_read` must also be set to true. This is a limitation of the Azure REST API
        #[builder(into, default)]
        pub registry_write: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Adds `ServiceConnect` permission to this Shared Access Account. It allows sending and receiving on the cloud-side endpoints.
        #[builder(into, default)]
        pub service_connect: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct SharedAccessPolicyResult {
        /// Adds `DeviceConnect` permission to this Shared Access Account. It allows sending and receiving on the device-side endpoints.
        ///
        /// > **NOTE** At least one of `registry_read`, `registry_write`, `service_connect`, `device_connect` permissions must be set to `true`.
        pub device_connect: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the IoTHub to which this Shared Access Policy belongs. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the IotHub Shared Access Policy resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Shared Access Policy.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary key used to create the authentication token.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        /// Adds `RegistryRead` permission to this Shared Access Account. It allows read access to the identity registry.
        pub registry_read: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Adds `RegistryWrite` permission to this Shared Access Account. It allows write access to the identity registry.
        ///
        /// > **NOTE** When `registry_write` is set to `true`, `registry_read` must also be set to true. This is a limitation of the Azure REST API
        pub registry_write: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub Shared Access Policy resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Shared Access Policy.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary key used to create the authentication token.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
        /// Adds `ServiceConnect` permission to this Shared Access Account. It allows sending and receiving on the cloud-side endpoints.
        pub service_connect: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedAccessPolicyArgs,
    ) -> SharedAccessPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let device_connect_binding = args.device_connect.get_output(context);
        let iothub_name_binding = args.iothub_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let registry_read_binding = args.registry_read.get_output(context);
        let registry_write_binding = args.registry_write.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_connect_binding = args.service_connect.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/sharedAccessPolicy:SharedAccessPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceConnect".into(),
                    value: device_connect_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubName".into(),
                    value: iothub_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryRead".into(),
                    value: registry_read_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryWrite".into(),
                    value: registry_write_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceConnect".into(),
                    value: service_connect_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedAccessPolicyResult {
            device_connect: o.get_field("deviceConnect"),
            iothub_name: o.get_field("iothubName"),
            name: o.get_field("name"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            primary_key: o.get_field("primaryKey"),
            registry_read: o.get_field("registryRead"),
            registry_write: o.get_field("registryWrite"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            secondary_key: o.get_field("secondaryKey"),
            service_connect: o.get_field("serviceConnect"),
        }
    }
}
