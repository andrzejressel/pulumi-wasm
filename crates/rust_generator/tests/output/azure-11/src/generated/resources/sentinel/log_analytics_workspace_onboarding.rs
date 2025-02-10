/// Manages a Security Insights Sentinel Onboarding.
///
/// ## Example Usage
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_analytics_workspace_onboarding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogAnalyticsWorkspaceOnboardingArgs {
        /// Specifies if the Workspace is using Customer managed key. Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** To set up Microsoft Sentinel customer-managed key it needs to enable CMK on the workspace and add access policy to your Azure Key Vault. Details could be found on [this document](https://learn.microsoft.com/en-us/azure/sentinel/customer-managed-keys)
        ///
        /// > **Note:** Once a workspace is onboarded to Microsoft Sentinel with `customer_managed_key_enabled` set to true, it will not be able to be onboarded again with `customer_managed_key_enabled` set to false.
        #[builder(into, default)]
        pub customer_managed_key_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the Workspace Id. Changing this forces the Log Analytics Workspace off the board and onboard again. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LogAnalyticsWorkspaceOnboardingResult {
        /// Specifies if the Workspace is using Customer managed key. Defaults to `false`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** To set up Microsoft Sentinel customer-managed key it needs to enable CMK on the workspace and add access policy to your Azure Key Vault. Details could be found on [this document](https://learn.microsoft.com/en-us/azure/sentinel/customer-managed-keys)
        ///
        /// > **Note:** Once a workspace is onboarded to Microsoft Sentinel with `customer_managed_key_enabled` set to true, it will not be able to be onboarded again with `customer_managed_key_enabled` set to false.
        pub customer_managed_key_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Workspace Id. Changing this forces the Log Analytics Workspace off the board and onboard again. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogAnalyticsWorkspaceOnboardingArgs,
    ) -> LogAnalyticsWorkspaceOnboardingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let customer_managed_key_enabled_binding = args
            .customer_managed_key_enabled
            .get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:sentinel/logAnalyticsWorkspaceOnboarding:LogAnalyticsWorkspaceOnboarding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKeyEnabled".into(),
                    value: customer_managed_key_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogAnalyticsWorkspaceOnboardingResult {
            customer_managed_key_enabled: o.get_field("customerManagedKeyEnabled"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
