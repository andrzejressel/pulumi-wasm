/// Manages an IotHub Storage Container Endpoint
///
/// > **NOTE:** Endpoints can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azurerm_iothub_endpoint_*` resources - but the two ways of defining the endpoints cannot be used together. If both are used against the same IoTHub, spurious changes will occur. Also, defining a `azurerm_iothub_endpoint_*` resource and another endpoint of a different type directly on the `azure.iot.IoTHub` resource is not supported.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: acctestcont
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
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
///   exampleEndpointStorageContainer:
///     type: azure:iot:EndpointStorageContainer
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubId: ${exampleIoTHub.id}
///       name: acctest
///       containerName: acctestcont
///       connectionString: ${exampleAccount.primaryBlobConnectionString}
///       fileNameFormat: '{iothub}/{partition}_{YYYY}_{MM}_{DD}_{HH}_{mm}'
///       batchFrequencyInSeconds: 60
///       maxChunkSizeInBytes: 1.048576e+07
///       encoding: JSON
/// ```
///
/// ## Import
///
/// IoTHub Storage Container Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/endpointStorageContainer:EndpointStorageContainer storage_container1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/endpoints/storage_container_endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_storage_container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointStorageContainerArgs {
        /// Type used to authenticate against the storage endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        #[builder(into, default)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds.
        #[builder(into, default)]
        pub batch_frequency_in_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        #[builder(into, default)]
        pub connection_string: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of storage container in the storage account.
        #[builder(into)]
        pub container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Encoding that is used to serialize messages to blobs. Supported values are `Avro`, `AvroDeflate` and `JSON`. Default value is `Avro`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encoding: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URI of the Storage Container endpoint. This corresponds to the `primary_blob_endpoint` of the parent storage account. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        #[builder(into, default)]
        pub endpoint_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// File name format for the blob. All parameters are mandatory but can be reordered. Defaults to `{iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}`.
        #[builder(into, default)]
        pub file_name_format: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the storage endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        #[builder(into, default)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB).
        #[builder(into, default)]
        pub max_chunk_size_in_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Storage Container has been created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointStorageContainerResult {
        /// Type used to authenticate against the storage endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        pub authentication_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds.
        pub batch_frequency_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        pub connection_string: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of storage container in the storage account.
        pub container_name: pulumi_gestalt_rust::Output<String>,
        /// Encoding that is used to serialize messages to blobs. Supported values are `Avro`, `AvroDeflate` and `JSON`. Default value is `Avro`. Changing this forces a new resource to be created.
        pub encoding: pulumi_gestalt_rust::Output<Option<String>>,
        /// URI of the Storage Container endpoint. This corresponds to the `primary_blob_endpoint` of the parent storage account. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        pub endpoint_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// File name format for the blob. All parameters are mandatory but can be reordered. Defaults to `{iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}`.
        pub file_name_format: pulumi_gestalt_rust::Output<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the storage endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        pub identity_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_gestalt_rust::Output<String>,
        /// Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB).
        pub max_chunk_size_in_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the Storage Container has been created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EndpointStorageContainerArgs,
    ) -> EndpointStorageContainerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_type_binding = args.authentication_type.get_output(context);
        let batch_frequency_in_seconds_binding = args
            .batch_frequency_in_seconds
            .get_output(context);
        let connection_string_binding = args.connection_string.get_output(context);
        let container_name_binding = args.container_name.get_output(context);
        let encoding_binding = args.encoding.get_output(context);
        let endpoint_uri_binding = args.endpoint_uri.get_output(context);
        let file_name_format_binding = args.file_name_format.get_output(context);
        let identity_id_binding = args.identity_id.get_output(context);
        let iothub_id_binding = args.iothub_id.get_output(context);
        let max_chunk_size_in_bytes_binding = args
            .max_chunk_size_in_bytes
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:iot/endpointStorageContainer:EndpointStorageContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationType".into(),
                    value: authentication_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchFrequencyInSeconds".into(),
                    value: batch_frequency_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionString".into(),
                    value: connection_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerName".into(),
                    value: container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encoding".into(),
                    value: encoding_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointUri".into(),
                    value: endpoint_uri_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fileNameFormat".into(),
                    value: file_name_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: identity_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iothubId".into(),
                    value: iothub_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxChunkSizeInBytes".into(),
                    value: max_chunk_size_in_bytes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EndpointStorageContainerResult {
            authentication_type: o.get_field("authenticationType"),
            batch_frequency_in_seconds: o.get_field("batchFrequencyInSeconds"),
            connection_string: o.get_field("connectionString"),
            container_name: o.get_field("containerName"),
            encoding: o.get_field("encoding"),
            endpoint_uri: o.get_field("endpointUri"),
            file_name_format: o.get_field("fileNameFormat"),
            identity_id: o.get_field("identityId"),
            iothub_id: o.get_field("iothubId"),
            max_chunk_size_in_bytes: o.get_field("maxChunkSizeInBytes"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
