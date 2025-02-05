/// Manages a Function App Active Slot.
///
/// ## Example Usage
///
/// ### Windows Function App
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("windowsfunctionappsa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionAppActiveSlot = function_app_active_slot::create(
///         "exampleFunctionAppActiveSlot",
///         FunctionAppActiveSlotArgs::builder()
///             .slot_id("${exampleWindowsFunctionAppSlot.id}")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .os_type("Windows")
///             .resource_group_name("${example.name}")
///             .sku_name("Y1")
///             .build_struct(),
///     );
///     let exampleWindowsFunctionApp = windows_function_app::create(
///         "exampleWindowsFunctionApp",
///         WindowsFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-windows-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(WindowsFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleWindowsFunctionAppSlot = windows_function_app_slot::create(
///         "exampleWindowsFunctionAppSlot",
///         WindowsFunctionAppSlotArgs::builder()
///             .function_app_id("${exampleWindowsFunctionApp.id}")
///             .name("example-windows-function-app-slot")
///             .site_config(WindowsFunctionAppSlotSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Linux Function App
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("linuxfunctionappsa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionAppActiveSlot = function_app_active_slot::create(
///         "exampleFunctionAppActiveSlot",
///         FunctionAppActiveSlotArgs::builder()
///             .slot_id("${exampleLinuxFunctionAppSlot.id}")
///             .build_struct(),
///     );
///     let exampleLinuxFunctionApp = linux_function_app::create(
///         "exampleLinuxFunctionApp",
///         LinuxFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-linux-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleLinuxFunctionAppSlot = linux_function_app_slot::create(
///         "exampleLinuxFunctionAppSlot",
///         LinuxFunctionAppSlotArgs::builder()
///             .function_app_id("${exampleLinuxFunctionApp.name}")
///             .name("example-linux-function-app-slot")
///             .site_config(LinuxFunctionAppSlotSiteConfig::builder().build_struct())
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("Y1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// a Function App Active Slot can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/functionAppActiveSlot:FunctionAppActiveSlot example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1"
/// ```
///
pub mod function_app_active_slot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionAppActiveSlotArgs {
        /// The swap action should overwrite the Production slot's network configuration with the configuration from this slot. Defaults to `true`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub overwrite_network_config: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Slot to swap with `Production`.
        #[builder(into)]
        pub slot_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionAppActiveSlotResult {
        /// The timestamp of the last successful swap with `Production`
        pub last_successful_swap: pulumi_wasm_rust::Output<String>,
        /// The swap action should overwrite the Production slot's network configuration with the configuration from this slot. Defaults to `true`. Changing this forces a new resource to be created.
        pub overwrite_network_config: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Slot to swap with `Production`.
        pub slot_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FunctionAppActiveSlotArgs,
    ) -> FunctionAppActiveSlotResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let overwrite_network_config_binding = args
            .overwrite_network_config
            .get_output(context)
            .get_inner();
        let slot_id_binding = args.slot_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/functionAppActiveSlot:FunctionAppActiveSlot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "overwriteNetworkConfig".into(),
                    value: &overwrite_network_config_binding,
                },
                register_interface::ObjectField {
                    name: "slotId".into(),
                    value: &slot_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FunctionAppActiveSlotResult {
            last_successful_swap: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastSuccessfulSwap"),
            ),
            overwrite_network_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("overwriteNetworkConfig"),
            ),
            slot_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("slotId")),
        }
    }
}
