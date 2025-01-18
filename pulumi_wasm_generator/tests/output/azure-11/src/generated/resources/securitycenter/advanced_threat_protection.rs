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
pub mod advanced_threat_protection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AdvancedThreatProtectionArgs {
        /// Should Advanced Threat Protection be enabled on this resource?
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Azure Resource which to enable Advanced Threat Protection on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AdvancedThreatProtectionResult {
        /// Should Advanced Threat Protection be enabled on this resource?
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The ID of the Azure Resource which to enable Advanced Threat Protection on. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AdvancedThreatProtectionArgs,
    ) -> AdvancedThreatProtectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let target_resource_id_binding = args.target_resource_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/advancedThreatProtection:AdvancedThreatProtection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "targetResourceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AdvancedThreatProtectionResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetResourceId").unwrap(),
            ),
        }
    }
}
