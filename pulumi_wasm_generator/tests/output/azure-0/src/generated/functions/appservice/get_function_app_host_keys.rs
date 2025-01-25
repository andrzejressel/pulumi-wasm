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
            results: Vec::from([
                register_interface::ResultField {
                    name: "blobsExtensionKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultFunctionKey".into(),
                },
                register_interface::ResultField {
                    name: "durabletaskExtensionKey".into(),
                },
                register_interface::ResultField {
                    name: "eventGridExtensionConfigKey".into(),
                },
                register_interface::ResultField {
                    name: "eventGridExtensionKey".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "signalrExtensionKey".into(),
                },
                register_interface::ResultField {
                    name: "webpubsubExtensionKey".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFunctionAppHostKeysResult {
            blobs_extension_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobsExtensionKey").unwrap(),
            ),
            default_function_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultFunctionKey").unwrap(),
            ),
            durabletask_extension_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("durabletaskExtensionKey").unwrap(),
            ),
            event_grid_extension_config_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventGridExtensionConfigKey").unwrap(),
            ),
            event_grid_extension_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventGridExtensionKey").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            signalr_extension_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signalrExtensionKey").unwrap(),
            ),
            webpubsub_extension_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webpubsubExtensionKey").unwrap(),
            ),
        }
    }
}
