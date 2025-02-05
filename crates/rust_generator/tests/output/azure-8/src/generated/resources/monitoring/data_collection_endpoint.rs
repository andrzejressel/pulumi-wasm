/// Manages a Data Collection Endpoint.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleDataCollectionEndpoint:
///     type: azure:monitoring:DataCollectionEndpoint
///     name: example
///     properties:
///       name: example-mdce
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       kind: Windows
///       publicNetworkAccessEnabled: true
///       description: monitor_data_collection_endpoint example
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Data Collection Endpoints can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/dataCollectionEndpoint:DataCollectionEndpoint example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Insights/dataCollectionEndpoints/endpoint1
/// ```
///
pub mod data_collection_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCollectionEndpointArgs {
        /// Specifies a description for the Data Collection Endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The kind of the Data Collection Endpoint. Possible values are `Linux` and `Windows`.
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Data Collection Endpoint should exist. Changing this forces a new Data Collection Endpoint to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Collection Endpoint. Changing this forces a new Data Collection Endpoint to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether network access from public internet to the Data Collection Endpoint are allowed. Possible values are `true` and `false`. Default to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Data Collection Endpoint should exist. Changing this forces a new Data Collection Endpoint to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Data Collection Endpoint.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCollectionEndpointResult {
        /// The endpoint used for accessing configuration, e.g., `https://mydce-abcd.eastus-1.control.monitor.azure.com`.
        pub configuration_access_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies a description for the Data Collection Endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The immutable ID of the Data Collection Endpoint.
        pub immutable_id: pulumi_wasm_rust::Output<String>,
        /// The kind of the Data Collection Endpoint. Possible values are `Linux` and `Windows`.
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Data Collection Endpoint should exist. Changing this forces a new Data Collection Endpoint to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The endpoint used for ingesting logs, e.g., `https://mydce-abcd.eastus-1.ingest.monitor.azure.com`.
        pub logs_ingestion_endpoint: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Collection Endpoint. Changing this forces a new Data Collection Endpoint to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether network access from public internet to the Data Collection Endpoint are allowed. Possible values are `true` and `false`. Default to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Data Collection Endpoint should exist. Changing this forces a new Data Collection Endpoint to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Data Collection Endpoint.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataCollectionEndpointArgs,
    ) -> DataCollectionEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let kind_binding = args.kind.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/dataCollectionEndpoint:DataCollectionEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
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
        let o = register_interface::register(context.get_inner(), &request);
        DataCollectionEndpointResult {
            configuration_access_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationAccessEndpoint"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            immutable_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("immutableId"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logs_ingestion_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logsIngestionEndpoint"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
