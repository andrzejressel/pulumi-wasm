pub mod get_function_app_host_keys {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFunctionAppHostKeysArgs {
        /// The name of the Function App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Function App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFunctionAppHostKeysResult {
        /// Function App resource's Blobs Extension system key.
        pub blobs_extension_key: pulumi_wasm_rust::Output<String>,
        /// Function App resource's default function key.
        pub default_function_key: pulumi_wasm_rust::Output<String>,
        /// Function App resource's Durable Task Extension system key.
        pub durabletask_extension_key: pulumi_wasm_rust::Output<String>,
        /// Function App resource's Event Grid Extension Config system key.
        pub event_grid_extension_config_key: pulumi_wasm_rust::Output<String>,
        pub event_grid_extension_key: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Function App resource's secret key
        pub primary_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Function App resource's SignalR Extension system key.
        pub signalr_extension_key: pulumi_wasm_rust::Output<String>,
        /// Function App resource's Web PubSub Extension system key.
        pub webpubsub_extension_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFunctionAppHostKeysArgs,
    ) -> GetFunctionAppHostKeysResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getFunctionAppHostKeys:getFunctionAppHostKeys"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFunctionAppHostKeysResult {
            blobs_extension_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blobsExtensionKey"),
            ),
            default_function_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultFunctionKey"),
            ),
            durabletask_extension_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("durabletaskExtensionKey"),
            ),
            event_grid_extension_config_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventGridExtensionConfigKey"),
            ),
            event_grid_extension_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("eventGridExtensionKey"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryKey"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            signalr_extension_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signalrExtensionKey"),
            ),
            webpubsub_extension_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("webpubsubExtensionKey"),
            ),
        }
    }
}
