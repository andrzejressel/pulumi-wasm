/// Manages an App Service Static Web App Function App Registration.
///
/// > **NOTE:** This resource registers the specified Function App to the `Production` build of the Static Web App.
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
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("examplesstorageacc")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleLinuxFunctionApp = linux_function_app::create(
///         "exampleLinuxFunctionApp",
///         LinuxFunctionAppArgs::builder()
///             .location("${example.location}")
///             .name("example-function-app")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxFunctionAppSiteConfig::builder().build_struct())
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example-service-plan")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("S1")
///             .build_struct(),
///     );
///     let exampleStaticWebApp = static_web_app::create(
///         "exampleStaticWebApp",
///         StaticWebAppArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleStaticWebAppFunctionAppRegistration = static_web_app_function_app_registration::create(
///         "exampleStaticWebAppFunctionAppRegistration",
///         StaticWebAppFunctionAppRegistrationArgs::builder()
///             .function_app_id("${exampleLinuxFunctionApp.id}")
///             .static_web_app_id("${exampleStaticWebApp.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Static Web App Function App Registration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/staticWebAppFunctionAppRegistration:StaticWebAppFunctionAppRegistration example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Web/staticSites/my-static-site1/userProvidedFunctionApps/myFunctionApp
/// ```
///
pub mod static_web_app_function_app_registration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticWebAppFunctionAppRegistrationArgs {
        /// The ID of a Linux or Windows Function App to connect to the Static Web App as a Backend. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only one Function App can be connected to a Static Web App. Multiple Function Apps are not currently supported.
        ///
        /// > **NOTE:** Connecting a Function App resource to a Static Web App resource updates the Function App to use AuthV2 and configures the `azure_static_web_app_v2` which may need to be accounted for by the use of `ignore_changes` depending on the existing `auth_settings_v2` configuration of the target Function App.
        #[builder(into)]
        pub function_app_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Static Web App to register the Function App to as a backend. Changing this forces a new resource to be created.
        #[builder(into)]
        pub static_web_app_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StaticWebAppFunctionAppRegistrationResult {
        /// The ID of a Linux or Windows Function App to connect to the Static Web App as a Backend. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Only one Function App can be connected to a Static Web App. Multiple Function Apps are not currently supported.
        ///
        /// > **NOTE:** Connecting a Function App resource to a Static Web App resource updates the Function App to use AuthV2 and configures the `azure_static_web_app_v2` which may need to be accounted for by the use of `ignore_changes` depending on the existing `auth_settings_v2` configuration of the target Function App.
        pub function_app_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Static Web App to register the Function App to as a backend. Changing this forces a new resource to be created.
        pub static_web_app_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StaticWebAppFunctionAppRegistrationArgs,
    ) -> StaticWebAppFunctionAppRegistrationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_app_id_binding = args
            .function_app_id
            .get_output(context)
            .get_inner();
        let static_web_app_id_binding = args
            .static_web_app_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/staticWebAppFunctionAppRegistration:StaticWebAppFunctionAppRegistration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionAppId".into(),
                    value: &function_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "staticWebAppId".into(),
                    value: &static_web_app_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StaticWebAppFunctionAppRegistrationResult {
            function_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("functionAppId"),
            ),
            static_web_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("staticWebAppId"),
            ),
        }
    }
}
