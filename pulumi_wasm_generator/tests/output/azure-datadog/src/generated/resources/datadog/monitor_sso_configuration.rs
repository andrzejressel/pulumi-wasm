/// Manages SingleSignOn on the datadog Monitor.
///
/// ## Example Usage
///
/// ### Enabling SSO on monitor
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-datadog
///       location: West US 2
///   exampleMonitor:
///     type: azure:datadog:Monitor
///     name: example
///     properties:
///       name: example-monitor
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       datadogOrganization:
///         apiKey: XXXX
///         applicationKey: XXXX
///       user:
///         name: Example
///         email: abc@xyz.com
///       skuName: Linked
///       identity:
///         type: SystemAssigned
///   exampleMonitorSsoConfiguration:
///     type: azure:datadog:MonitorSsoConfiguration
///     name: example
///     properties:
///       datadogMonitorId: ${exampleMonitor.id}
///       singleSignOnEnabled: Enable
///       enterpriseApplicationId: XXXX
/// ```
///
/// ## Import
///
/// SingleSignOn on the Datadog Monitor can be imported using the `signle sign on resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datadog/monitorSsoConfiguration:MonitorSsoConfiguration example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Datadog/monitors/monitor1/singleSignOnConfigurations/default
/// ```
///
pub mod monitor_sso_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MonitorSsoConfigurationArgs {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor SSO Configuration. Changing this forces a new Datadog Monitor SSO Configuration to be created.
        #[builder(into)]
        pub datadog_monitor_id: pulumi_wasm_rust::Output<String>,
        /// The application Id to perform SSO operation.
        #[builder(into)]
        pub enterprise_application_id: pulumi_wasm_rust::Output<String>,
        /// The name of the SingleSignOn configuration. Defaults to `default`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The state of SingleSignOn configuration. Possible values are `Enable` and `Disable`.
        #[builder(into)]
        pub single_sign_on_enabled: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MonitorSsoConfigurationResult {
        /// The Datadog Monitor Id which should be used for this Datadog Monitor SSO Configuration. Changing this forces a new Datadog Monitor SSO Configuration to be created.
        pub datadog_monitor_id: pulumi_wasm_rust::Output<String>,
        /// The application Id to perform SSO operation.
        pub enterprise_application_id: pulumi_wasm_rust::Output<String>,
        /// The SingleSignOn URL to login to Datadog org.
        pub login_url: pulumi_wasm_rust::Output<String>,
        /// The name of the SingleSignOn configuration. Defaults to `default`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The state of SingleSignOn configuration. Possible values are `Enable` and `Disable`.
        pub single_sign_on_enabled: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MonitorSsoConfigurationArgs,
    ) -> MonitorSsoConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let datadog_monitor_id_binding = args.datadog_monitor_id.get_inner();
        let enterprise_application_id_binding = args
            .enterprise_application_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let single_sign_on_enabled_binding = args.single_sign_on_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datadog/monitorSsoConfiguration:MonitorSsoConfiguration"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "datadogMonitorId".into(),
                    value: &datadog_monitor_id_binding,
                },
                register_interface::ObjectField {
                    name: "enterpriseApplicationId".into(),
                    value: &enterprise_application_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "singleSignOnEnabled".into(),
                    value: &single_sign_on_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "datadogMonitorId".into(),
                },
                register_interface::ResultField {
                    name: "enterpriseApplicationId".into(),
                },
                register_interface::ResultField {
                    name: "loginUrl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "singleSignOnEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MonitorSsoConfigurationResult {
            datadog_monitor_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datadogMonitorId").unwrap(),
            ),
            enterprise_application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enterpriseApplicationId").unwrap(),
            ),
            login_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loginUrl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            single_sign_on_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("singleSignOnEnabled").unwrap(),
            ),
        }
    }
}