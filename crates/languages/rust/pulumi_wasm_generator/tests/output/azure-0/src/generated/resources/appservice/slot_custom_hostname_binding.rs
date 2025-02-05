/// Manages a Hostname Binding within an App Service Slot.
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
///             .name("some-resource-group")
///             .build_struct(),
///     );
///     let exampleAppService = app_service::create(
///         "exampleAppService",
///         AppServiceArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("some-app-service")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("some-app-service-plan")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
///     let exampleSlot = slot::create(
///         "exampleSlot",
///         SlotArgs::builder()
///             .app_service_name("${exampleAppService.name}")
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("staging")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSlotCustomHostnameBinding = slot_custom_hostname_binding::create(
///         "exampleSlotCustomHostnameBinding",
///         SlotCustomHostnameBindingArgs::builder()
///             .app_service_slot_id("${exampleSlot.id}")
///             .hostname("www.mywebsite.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Custom Hostname Bindings can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/slotCustomHostnameBinding:SlotCustomHostnameBinding mywebsite /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1/slots/staging/hostNameBindings/mywebsite.com
/// ```
///
pub mod slot_custom_hostname_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotCustomHostnameBindingArgs {
        /// The ID of the App Service Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_slot_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Custom Hostname to use for the App Service, example `www.example.com`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A CNAME needs to be configured from this Hostname to the Azure Website - otherwise Azure will reject the Hostname Binding.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::InputOrOutput<String>,
        /// The SSL type. Possible values are `IpBasedEnabled` and `SniEnabled`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ssl_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The SSL certificate thumbprint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `thumbprint` must be specified when `ssl_state` is set.
        #[builder(into, default)]
        pub thumbprint: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SlotCustomHostnameBindingResult {
        /// The ID of the App Service Slot. Changing this forces a new resource to be created.
        pub app_service_slot_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Custom Hostname to use for the App Service, example `www.example.com`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** A CNAME needs to be configured from this Hostname to the Azure Website - otherwise Azure will reject the Hostname Binding.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The SSL type. Possible values are `IpBasedEnabled` and `SniEnabled`. Changing this forces a new resource to be created.
        pub ssl_state: pulumi_wasm_rust::Output<String>,
        /// The SSL certificate thumbprint. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `thumbprint` must be specified when `ssl_state` is set.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
        /// The virtual IP address assigned to the hostname if IP based SSL is enabled.
        pub virtual_ip: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SlotCustomHostnameBindingArgs,
    ) -> SlotCustomHostnameBindingResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let app_service_slot_id_binding = args
            .app_service_slot_id
            .get_output(context)
            .get_inner();
        let hostname_binding = args.hostname.get_output(context).get_inner();
        let ssl_state_binding = args.ssl_state.get_output(context).get_inner();
        let thumbprint_binding = args.thumbprint.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/slotCustomHostnameBinding:SlotCustomHostnameBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceSlotId".into(),
                    value: &app_service_slot_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "sslState".into(),
                    value: &ssl_state_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SlotCustomHostnameBindingResult {
            app_service_slot_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appServiceSlotId"),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            ssl_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sslState"),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
            virtual_ip: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualIp"),
            ),
        }
    }
}
