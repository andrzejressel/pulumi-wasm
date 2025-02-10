#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_function_app_host_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionAppHostKeysArgs {
        /// The name of the Function App.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Function App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionAppHostKeysResult {
        /// Function App resource's Blobs Extension system key.
        pub blobs_extension_key: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's default function key.
        pub default_function_key: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's Durable Task Extension system key.
        pub durabletask_extension_key: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's Event Grid Extension Config system key.
        pub event_grid_extension_config_key: pulumi_gestalt_rust::Output<String>,
        pub event_grid_extension_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's secret key
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's SignalR Extension system key.
        pub signalr_extension_key: pulumi_gestalt_rust::Output<String>,
        /// Function App resource's Web PubSub Extension system key.
        pub webpubsub_extension_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFunctionAppHostKeysArgs,
    ) -> GetFunctionAppHostKeysResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getFunctionAppHostKeys:getFunctionAppHostKeys"
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
        GetFunctionAppHostKeysResult {
            blobs_extension_key: o.get_field("blobsExtensionKey"),
            default_function_key: o.get_field("defaultFunctionKey"),
            durabletask_extension_key: o.get_field("durabletaskExtensionKey"),
            event_grid_extension_config_key: o.get_field("eventGridExtensionConfigKey"),
            event_grid_extension_key: o.get_field("eventGridExtensionKey"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            primary_key: o.get_field("primaryKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            signalr_extension_key: o.get_field("signalrExtensionKey"),
            webpubsub_extension_key: o.get_field("webpubsubExtensionKey"),
        }
    }
}
