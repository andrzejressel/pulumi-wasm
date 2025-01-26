/// Manages a Security Insights Sentinel Onboarding.
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
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-law")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .customer_managed_key_enabled(false)
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Security Insights Sentinel Onboarding States can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/logAnalyticsWorkspaceOnboarding:LogAnalyticsWorkspaceOnboarding example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/onboardingStates/defaults
/// ```
///
pub mod log_analytics_workspace_onboarding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogAnalyticsWorkspaceOnboardingArgs {
        /// Specifies if the Workspace is using Customer managed key. Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** To set up Microsoft Sentinel customer-managed key it needs to enable CMK on the workspace and add access policy to your Azure Key Vault. Details could be found on [this document](https://learn.microsoft.com/en-us/azure/sentinel/customer-managed-keys)
        ///
        /// > **Note:** Once a workspace is onboarded to Microsoft Sentinel with `customer_managed_key_enabled` set to true, it will not be able to be onboarded again with `customer_managed_key_enabled` set to false.
        #[builder(into, default)]
        pub customer_managed_key_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Workspace Id. Changing this forces the Log Analytics Workspace off the board and onboard again. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogAnalyticsWorkspaceOnboardingResult {
        /// Specifies if the Workspace is using Customer managed key. Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** To set up Microsoft Sentinel customer-managed key it needs to enable CMK on the workspace and add access policy to your Azure Key Vault. Details could be found on [this document](https://learn.microsoft.com/en-us/azure/sentinel/customer-managed-keys)
        ///
        /// > **Note:** Once a workspace is onboarded to Microsoft Sentinel with `customer_managed_key_enabled` set to true, it will not be able to be onboarded again with `customer_managed_key_enabled` set to false.
        pub customer_managed_key_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Workspace Id. Changing this forces the Log Analytics Workspace off the board and onboard again. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LogAnalyticsWorkspaceOnboardingArgs,
    ) -> LogAnalyticsWorkspaceOnboardingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let customer_managed_key_enabled_binding = args
            .customer_managed_key_enabled
            .get_output(context)
            .get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/logAnalyticsWorkspaceOnboarding:LogAnalyticsWorkspaceOnboarding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customerManagedKeyEnabled".into(),
                    value: &customer_managed_key_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LogAnalyticsWorkspaceOnboardingResult {
            customer_managed_key_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerManagedKeyEnabled"),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
