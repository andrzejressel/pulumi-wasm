#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_collection_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataCollectionEndpointArgs {
        /// Specifies the name of the Data Collection Endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Data Collection Endpoint is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataCollectionEndpointResult {
        /// The endpoint used for accessing configuration, e.g., `https://mydce-abcd.eastus-1.control.monitor.azure.com`.
        pub configuration_access_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies a description for the Data Collection Endpoint.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The immutable ID of the Data Collection Endpoint.
        pub immutable_id: pulumi_gestalt_rust::Output<String>,
        /// The kind of the Data Collection Endpoint. Possible values are `Linux` and `Windows`.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Data Collection Endpoint should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The endpoint used for ingesting logs, e.g., `https://mydce-abcd.eastus-1.ingest.monitor.azure.com`.
        pub logs_ingestion_endpoint: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether network access from public internet to the Data Collection Endpoint are allowed. Possible values are `true` and `false`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Data Collection Endpoint.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDataCollectionEndpointArgs,
    ) -> GetDataCollectionEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:monitoring/getDataCollectionEndpoint:getDataCollectionEndpoint"
                .into(),
            version: super::super::super::get_version(),
            object: &[
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
        let o = context.invoke_resource(request);
        GetDataCollectionEndpointResult {
            configuration_access_endpoint: o.get_field("configurationAccessEndpoint"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            immutable_id: o.get_field("immutableId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            logs_ingestion_endpoint: o.get_field("logsIngestionEndpoint"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
