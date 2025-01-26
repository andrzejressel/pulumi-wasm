/// Manages a Web App Hybrid Connection.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleHybridConnection = hybrid_connection::create(
///         "exampleHybridConnection",
///         HybridConnectionArgs::builder()
///             .name("examplerhc1")
///             .relay_namespace_name("${exampleNamespace.name}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .location("${example.location}")
///             .name("example-relay")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("S1")
///             .build_struct(),
///     );
///     let exampleWebAppHybridConnection = web_app_hybrid_connection::create(
///         "exampleWebAppHybridConnection",
///         WebAppHybridConnectionArgs::builder()
///             .hostname("myhostname.example")
///             .port(8081)
///             .relay_id("${exampleHybridConnection.id}")
///             .web_app_id("${exampleWindowsWebApp.id}")
///             .build_struct(),
///     );
///     let exampleWindowsWebApp = windows_web_app::create(
///         "exampleWindowsWebApp",
///         WindowsWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example-web-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// a Web App Hybrid Connection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/webAppHybridConnection:WebAppHybridConnection example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1/hybridConnectionNamespaces/hybridConnectionNamespace1/relays/relay1"
/// ```
///
pub mod web_app_hybrid_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebAppHybridConnectionArgs {
        /// The hostname of the endpoint.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::InputOrOutput<String>,
        /// The port to use for the endpoint.
        #[builder(into)]
        pub port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The ID of the Relay Hybrid Connection to use. Changing this forces a new resource to be created.
        #[builder(into)]
        pub relay_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Relay key with `Send` permission to use. Defaults to `RootManageSharedAccessKey`
        #[builder(into, default)]
        pub send_key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Web App for this Hybrid Connection. Changing this forces a new resource to be created.
        #[builder(into)]
        pub web_app_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebAppHybridConnectionResult {
        /// The hostname of the endpoint.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The name of the Relay Namespace.
        pub namespace_name: pulumi_wasm_rust::Output<String>,
        /// The port to use for the endpoint.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the Relay Hybrid Connection to use. Changing this forces a new resource to be created.
        pub relay_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Relay in use.
        pub relay_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Relay key with `Send` permission to use. Defaults to `RootManageSharedAccessKey`
        pub send_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Primary Access Key for the `send_key_name`
        pub send_key_value: pulumi_wasm_rust::Output<String>,
        /// The Service Bus Namespace.
        pub service_bus_namespace: pulumi_wasm_rust::Output<String>,
        /// The suffix for the endpoint.
        pub service_bus_suffix: pulumi_wasm_rust::Output<String>,
        /// The ID of the Web App for this Hybrid Connection. Changing this forces a new resource to be created.
        pub web_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WebAppHybridConnectionArgs,
    ) -> WebAppHybridConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let relay_id_binding = args.relay_id.get_output(context).get_inner();
        let send_key_name_binding = args.send_key_name.get_output(context).get_inner();
        let web_app_id_binding = args.web_app_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/webAppHybridConnection:WebAppHybridConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "relayId".into(),
                    value: &relay_id_binding,
                },
                register_interface::ObjectField {
                    name: "sendKeyName".into(),
                    value: &send_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "webAppId".into(),
                    value: &web_app_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "namespaceName".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "relayId".into(),
                },
                register_interface::ResultField {
                    name: "relayName".into(),
                },
                register_interface::ResultField {
                    name: "sendKeyName".into(),
                },
                register_interface::ResultField {
                    name: "sendKeyValue".into(),
                },
                register_interface::ResultField {
                    name: "serviceBusNamespace".into(),
                },
                register_interface::ResultField {
                    name: "serviceBusSuffix".into(),
                },
                register_interface::ResultField {
                    name: "webAppId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebAppHybridConnectionResult {
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceName").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            relay_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relayId").unwrap(),
            ),
            relay_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relayName").unwrap(),
            ),
            send_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendKeyName").unwrap(),
            ),
            send_key_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendKeyValue").unwrap(),
            ),
            service_bus_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceBusNamespace").unwrap(),
            ),
            service_bus_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceBusSuffix").unwrap(),
            ),
            web_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("webAppId").unwrap(),
            ),
        }
    }
}
