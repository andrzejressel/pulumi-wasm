/// Manages an IoT Hub Device Update Instance.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: East US
///   exampleIotHubDeviceUpdateAccount:
///     type: azure:iot:IotHubDeviceUpdateAccount
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleIotHubDeviceUpdateInstance:
///     type: azure:iot:IotHubDeviceUpdateInstance
///     name: example
///     properties:
///       name: example
///       deviceUpdateAccountId: ${exampleIotHubDeviceUpdateAccount.id}
///       iothubId: ${exampleIoTHub.id}
///       diagnosticEnabled: true
///       diagnosticStorageAccount:
///         connectionString: ${exampleAccount.primaryConnectionString}
///         id: ${exampleAccount.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// IoT Hub Device Update Instance can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubDeviceUpdateInstance:IotHubDeviceUpdateInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.DeviceUpdate/accounts/account1/instances/instance1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iot_hub_device_update_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateInstanceArgs {
        /// Specifies the ID of the IoT Hub Device Update Account where the IoT Hub Device Update Instance exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub device_update_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the diagnostic log collection is enabled. Possible values are `true` and `false`. Defaults to `false`.
        #[builder(into, default)]
        pub diagnostic_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A `diagnostic_storage_account` block as defined below.
        #[builder(into, default)]
        pub diagnostic_storage_account: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::iot::IotHubDeviceUpdateInstanceDiagnosticStorageAccount,
            >,
        >,
        /// Specifies the ID of the IoT Hub associated with the IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Instance.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IotHubDeviceUpdateInstanceResult {
        /// Specifies the ID of the IoT Hub Device Update Account where the IoT Hub Device Update Instance exists. Changing this forces a new resource to be created.
        pub device_update_account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the diagnostic log collection is enabled. Possible values are `true` and `false`. Defaults to `false`.
        pub diagnostic_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A `diagnostic_storage_account` block as defined below.
        pub diagnostic_storage_account: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::iot::IotHubDeviceUpdateInstanceDiagnosticStorageAccount,
            >,
        >,
        /// Specifies the ID of the IoT Hub associated with the IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this IoT Hub Device Update Instance. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the IoT Hub Device Update Instance.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IotHubDeviceUpdateInstanceArgs,
    ) -> IotHubDeviceUpdateInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let device_update_account_id_binding = args
            .device_update_account_id
            .get_output(context)
            .get_inner();
        let diagnostic_enabled_binding = args
            .diagnostic_enabled
            .get_output(context)
            .get_inner();
        let diagnostic_storage_account_binding = args
            .diagnostic_storage_account
            .get_output(context)
            .get_inner();
        let iothub_id_binding = args.iothub_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/iotHubDeviceUpdateInstance:IotHubDeviceUpdateInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deviceUpdateAccountId".into(),
                    value: &device_update_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "diagnosticEnabled".into(),
                    value: &diagnostic_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "diagnosticStorageAccount".into(),
                    value: &diagnostic_storage_account_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IotHubDeviceUpdateInstanceResult {
            device_update_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceUpdateAccountId"),
            ),
            diagnostic_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diagnosticEnabled"),
            ),
            diagnostic_storage_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diagnosticStorageAccount"),
            ),
            iothub_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
