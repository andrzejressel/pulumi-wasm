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
pub mod endpoint_storage_container {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointStorageContainerArgs {
        /// Type used to authenticate against the storage endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        #[builder(into, default)]
        pub authentication_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds.
        #[builder(into, default)]
        pub batch_frequency_in_seconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of storage container in the storage account.
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Encoding that is used to serialize messages to blobs. Supported values are `Avro`, `AvroDeflate` and `JSON`. Default value is `Avro`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encoding: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// URI of the Storage Container endpoint. This corresponds to the `primary_blob_endpoint` of the parent storage account. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        #[builder(into, default)]
        pub endpoint_uri: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// File name format for the blob. All parameters are mandatory but can be reordered. Defaults to `{iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}`.
        #[builder(into, default)]
        pub file_name_format: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the storage endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        #[builder(into, default)]
        pub identity_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB).
        #[builder(into, default)]
        pub max_chunk_size_in_bytes: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Storage Container has been created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointStorageContainerResult {
        /// Type used to authenticate against the storage endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        pub authentication_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Time interval at which blobs are written to storage. Value should be between 60 and 720 seconds. Default value is 300 seconds.
        pub batch_frequency_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of storage container in the storage account.
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// Encoding that is used to serialize messages to blobs. Supported values are `Avro`, `AvroDeflate` and `JSON`. Default value is `Avro`. Changing this forces a new resource to be created.
        pub encoding: pulumi_wasm_rust::Output<Option<String>>,
        /// URI of the Storage Container endpoint. This corresponds to the `primary_blob_endpoint` of the parent storage account. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        pub endpoint_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// File name format for the blob. All parameters are mandatory but can be reordered. Defaults to `{iothub}/{partition}/{YYYY}/{MM}/{DD}/{HH}/{mm}`.
        pub file_name_format: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the storage endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        pub identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// Maximum number of bytes for each blob written to storage. Value should be between 10485760(10MB) and 524288000(500MB). Default value is 314572800(300MB).
        pub max_chunk_size_in_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the Storage Container has been created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EndpointStorageContainerArgs,
    ) -> EndpointStorageContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let batch_frequency_in_seconds_binding = args
            .batch_frequency_in_seconds
            .get_output(context)
            .get_inner();
        let connection_string_binding = args
            .connection_string
            .get_output(context)
            .get_inner();
        let container_name_binding = args.container_name.get_output(context).get_inner();
        let encoding_binding = args.encoding.get_output(context).get_inner();
        let endpoint_uri_binding = args.endpoint_uri.get_output(context).get_inner();
        let file_name_format_binding = args
            .file_name_format
            .get_output(context)
            .get_inner();
        let identity_id_binding = args.identity_id.get_output(context).get_inner();
        let iothub_id_binding = args.iothub_id.get_output(context).get_inner();
        let max_chunk_size_in_bytes_binding = args
            .max_chunk_size_in_bytes
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/endpointStorageContainer:EndpointStorageContainer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "batchFrequencyInSeconds".into(),
                    value: &batch_frequency_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "encoding".into(),
                    value: &encoding_binding,
                },
                register_interface::ObjectField {
                    name: "endpointUri".into(),
                    value: &endpoint_uri_binding,
                },
                register_interface::ObjectField {
                    name: "fileNameFormat".into(),
                    value: &file_name_format_binding,
                },
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
                register_interface::ObjectField {
                    name: "maxChunkSizeInBytes".into(),
                    value: &max_chunk_size_in_bytes_binding,
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
                    name: "authenticationType".into(),
                },
                register_interface::ResultField {
                    name: "batchFrequencyInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "encoding".into(),
                },
                register_interface::ResultField {
                    name: "endpointUri".into(),
                },
                register_interface::ResultField {
                    name: "fileNameFormat".into(),
                },
                register_interface::ResultField {
                    name: "identityId".into(),
                },
                register_interface::ResultField {
                    name: "iothubId".into(),
                },
                register_interface::ResultField {
                    name: "maxChunkSizeInBytes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointStorageContainerResult {
            authentication_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationType").unwrap(),
            ),
            batch_frequency_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchFrequencyInSeconds").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            encoding: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encoding").unwrap(),
            ),
            endpoint_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUri").unwrap(),
            ),
            file_name_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileNameFormat").unwrap(),
            ),
            identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityId").unwrap(),
            ),
            iothub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubId").unwrap(),
            ),
            max_chunk_size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxChunkSizeInBytes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
