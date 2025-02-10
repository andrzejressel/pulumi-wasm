/// Manages a resources Advanced Threat Protection setting.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: atp-example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorage
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: example
///   exampleAdvancedThreatProtection:
///     type: azure:securitycenter:AdvancedThreatProtection
///     name: example
///     properties:
///       targetResourceId: ${exampleAccount.id}
///       enabled: true
/// ```
///
/// ## Import
///
/// Advanced Threat Protection can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/advancedThreatProtection:AdvancedThreatProtection example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/exampleResourceGroup/providers/Microsoft.Storage/storageAccounts/exampleaccount/providers/Microsoft.Security/advancedThreatProtectionSettings/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod advanced_threat_protection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AdvancedThreatProtectionArgs {
        /// Should Advanced Threat Protection be enabled on this resource?
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The ID of the Azure Resource which to enable Advanced Threat Protection on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AdvancedThreatProtectionResult {
        /// Should Advanced Threat Protection be enabled on this resource?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the Azure Resource which to enable Advanced Threat Protection on. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AdvancedThreatProtectionArgs,
    ) -> AdvancedThreatProtectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let target_resource_id_binding = args.target_resource_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:securitycenter/advancedThreatProtection:AdvancedThreatProtection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResourceId".into(),
                    value: target_resource_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AdvancedThreatProtectionResult {
            enabled: o.get_field("enabled"),
            target_resource_id: o.get_field("targetResourceId"),
        }
    }
}
