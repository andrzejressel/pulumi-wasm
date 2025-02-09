/// Manages an App Service Hybrid Connection for an existing App Service, Relay and Service Bus.
///
/// !> **NOTE:** This resource has been deprecated in version 5.0 of the provider and will be removed in version 6.0. Please use `azure.appservice.FunctionAppHybridConnection` and `azure.appservice.WebAppHybridConnection` resources instead.
///
/// ## Example Usage
///
/// This example provisions an App Service, a Relay Hybrid Connection, and a Service Bus using their outputs to create the App Service Hybrid Connection.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("exampleResourceGroup1")
///             .build_struct(),
///     );
///     let exampleAppService = app_service::create(
///         "exampleAppService",
///         AppServiceArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("exampleAppService1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleHybridConnection = hybrid_connection::create(
///         "exampleHybridConnection",
///         HybridConnectionArgs::builder()
///             .name("exampleRHC1")
///             .relay_namespace_name("${exampleNamespace.name}")
///             .resource_group_name("${example.name}")
///             .user_metadata("examplemetadata")
///             .build_struct(),
///     );
///     let exampleHybridConnection2 = hybrid_connection::create(
///         "exampleHybridConnection2",
///         HybridConnectionArgs::builder()
///             .app_service_name("${exampleAppService.name}")
///             .hostname("testhostname.example")
///             .port(8080)
///             .relay_id("${exampleHybridConnection.id}")
///             .resource_group_name("${example.name}")
///             .send_key_name("exampleSharedAccessKey")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder()
///             .location("${example.location}")
///             .name("exampleRN1")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("exampleAppServicePlan1")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Hybrid Connections can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/hybridConnection:HybridConnection example /subscriptions/00000000-0000-0000-0000-00000000000/resourceGroups/exampleResourceGroup1/providers/Microsoft.Web/sites/exampleAppService1/hybridConnectionNamespaces/exampleRN1/relays/exampleRHC1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hybrid_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HybridConnectionArgs {
        /// Specifies the name of the App Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The hostname of the endpoint.
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The port of the endpoint.
        #[builder(into)]
        pub port: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The ID of the Service Bus Relay. Changing this forces a new resource to be created.
        #[builder(into)]
        pub relay_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the App Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Service Bus key which has Send permissions. Defaults to `RootManageSharedAccessKey`.
        #[builder(into, default)]
        pub send_key_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HybridConnectionResult {
        /// Specifies the name of the App Service. Changing this forces a new resource to be created.
        pub app_service_name: pulumi_gestalt_rust::Output<String>,
        /// The hostname of the endpoint.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The name of the Relay Namespace.
        pub namespace_name: pulumi_gestalt_rust::Output<String>,
        /// The port of the endpoint.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the Service Bus Relay. Changing this forces a new resource to be created.
        pub relay_id: pulumi_gestalt_rust::Output<String>,
        pub relay_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the App Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Bus key which has Send permissions. Defaults to `RootManageSharedAccessKey`.
        pub send_key_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value of the Service Bus Primary Access key.
        pub send_key_value: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Bus namespace.
        pub service_bus_namespace: pulumi_gestalt_rust::Output<String>,
        /// The suffix for the service bus endpoint.
        pub service_bus_suffix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HybridConnectionArgs,
    ) -> HybridConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_name_binding = args.app_service_name.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let port_binding = args.port.get_output(context);
        let relay_id_binding = args.relay_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let send_key_name_binding = args.send_key_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/hybridConnection:HybridConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceName".into(),
                    value: app_service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relayId".into(),
                    value: relay_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendKeyName".into(),
                    value: send_key_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HybridConnectionResult {
            app_service_name: o.get_field("appServiceName"),
            hostname: o.get_field("hostname"),
            namespace_name: o.get_field("namespaceName"),
            port: o.get_field("port"),
            relay_id: o.get_field("relayId"),
            relay_name: o.get_field("relayName"),
            resource_group_name: o.get_field("resourceGroupName"),
            send_key_name: o.get_field("sendKeyName"),
            send_key_value: o.get_field("sendKeyValue"),
            service_bus_namespace: o.get_field("serviceBusNamespace"),
            service_bus_suffix: o.get_field("serviceBusSuffix"),
        }
    }
}
