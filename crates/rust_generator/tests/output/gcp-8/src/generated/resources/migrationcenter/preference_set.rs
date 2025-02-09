/// Manages the PreferenceSet resource.
///
///
/// To get more information about PreferenceSet, see:
///
/// * [API documentation](https://cloud.google.com/migration-center/docs/reference/rest/v1)
/// * How-to Guides
///     * [Managing Migration Preferences](https://cloud.google.com/migration-center/docs/migration-preferences)
///
/// ## Example Usage
///
/// ### Preference Set Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = preference_set::create(
///         "default",
///         PreferenceSetArgs::builder()
///             .description("Terraform integration test description")
///             .display_name("Terraform integration test display")
///             .location("us-central1")
///             .preference_set_id("preference-set-test")
///             .virtual_machine_preferences(
///                 PreferenceSetVirtualMachinePreferences::builder()
///                     .sizingOptimizationStrategy(
///                         "SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE",
///                     )
///                     .targetProduct("COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE")
///                     .vmwareEnginePreferences(
///                         PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences::builder()
///                             .cpuOvercommitRatio(1.5)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Preference Set Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = preference_set::create(
///         "default",
///         PreferenceSetArgs::builder()
///             .description("Terraform integration test description")
///             .display_name("Terraform integration test display")
///             .location("us-central1")
///             .preference_set_id("preference-set-test")
///             .virtual_machine_preferences(
///                 PreferenceSetVirtualMachinePreferences::builder()
///                     .commitmentPlan("COMMITMENT_PLAN_ONE_YEAR")
///                     .computeEnginePreferences(
///                         PreferenceSetVirtualMachinePreferencesComputeEnginePreferences::builder()
///                             .licenseType("LICENSE_TYPE_BRING_YOUR_OWN_LICENSE")
///                             .machinePreferences(
///                                 PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferences::builder()
///                                     .allowedMachineSeries(
///                                         vec![
///                                             PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferencesAllowedMachineSeries::builder()
///                                             .code("C3").build_struct(),
///                                         ],
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .regionPreferences(
///                         PreferenceSetVirtualMachinePreferencesRegionPreferences::builder()
///                             .preferredRegions(vec!["us-central1",])
///                             .build_struct(),
///                     )
///                     .sizingOptimizationStrategy(
///                         "SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE",
///                     )
///                     .soleTenancyPreferences(
///                         PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences::builder()
///                             .commitmentPlan("ON_DEMAND")
///                             .cpuOvercommitRatio(1.2)
///                             .hostMaintenancePolicy("HOST_MAINTENANCE_POLICY_DEFAULT")
///                             .nodeTypes(
///                                 vec![
///                                     PreferenceSetVirtualMachinePreferencesSoleTenancyPreferencesNodeType::builder()
///                                     .nodeName("tf-test").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .targetProduct("COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE")
///                     .vmwareEnginePreferences(
///                         PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences::builder()
///                             .commitmentPlan("ON_DEMAND")
///                             .cpuOvercommitRatio(1.5)
///                             .storageDeduplicationCompressionRatio(1.3)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PreferenceSet can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/preferenceSets/{{preference_set_id}}`
///
/// * `{{project}}/{{location}}/{{preference_set_id}}`
///
/// * `{{location}}/{{preference_set_id}}`
///
/// When using the `pulumi import` command, PreferenceSet can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:migrationcenter/preferenceSet:PreferenceSet default projects/{{project}}/locations/{{location}}/preferenceSets/{{preference_set_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:migrationcenter/preferenceSet:PreferenceSet default {{project}}/{{location}}/{{preference_set_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:migrationcenter/preferenceSet:PreferenceSet default {{location}}/{{preference_set_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod preference_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreferenceSetArgs {
        /// A description of the preference set.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-friendly display name. Maximum length is 63 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Part of `parent`. See documentation of `projectsId`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression `a-z?`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub preference_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VirtualMachinePreferences enables you to create sets of assumptions, for example, a geographical location and pricing track, for your migrated virtual machines. The set of preferences influence recommendations for migrating virtual machine assets.
        /// Structure is documented below.
        #[builder(into, default)]
        pub virtual_machine_preferences: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferences,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct PreferenceSetResult {
        /// Output only. The timestamp when the preference set was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the preference set.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User-friendly display name. Maximum length is 63 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Part of `parent`. See documentation of `projectsId`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. Name of the preference set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression `a-z?`.
        ///
        ///
        /// - - -
        pub preference_set_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. The timestamp when the preference set was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// VirtualMachinePreferences enables you to create sets of assumptions, for example, a geographical location and pricing track, for your migrated virtual machines. The set of preferences influence recommendations for migrating virtual machine assets.
        /// Structure is documented below.
        pub virtual_machine_preferences: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferences,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreferenceSetArgs,
    ) -> PreferenceSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let preference_set_id_binding = args.preference_set_id.get_output(context);
        let project_binding = args.project.get_output(context);
        let virtual_machine_preferences_binding = args
            .virtual_machine_preferences
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:migrationcenter/preferenceSet:PreferenceSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferenceSetId".into(),
                    value: preference_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualMachinePreferences".into(),
                    value: virtual_machine_preferences_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PreferenceSetResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            preference_set_id: o.get_field("preferenceSetId"),
            project: o.get_field("project"),
            update_time: o.get_field("updateTime"),
            virtual_machine_preferences: o.get_field("virtualMachinePreferences"),
        }
    }
}
