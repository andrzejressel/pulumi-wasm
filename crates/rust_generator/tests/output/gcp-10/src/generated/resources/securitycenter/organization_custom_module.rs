/// Represents an instance of a Security Health Analytics custom module, including
/// its full module name, display name, enablement state, and last updated time.
/// You can create a custom module at the organization, folder, or project level.
/// Custom modules that you create at the organization or folder level are inherited
/// by the child folders and projects.
///
///
/// To get more information about OrganizationCustomModule, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/organizations.securityHealthAnalyticsSettings.customModules)
/// * How-to Guides
///     * [Overview of custom modules for Security Health Analytics](https://cloud.google.com/security-command-center/docs/custom-modules-sha-overview)
///
/// ## Example Usage
///
/// ### Scc Organization Custom Module Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_custom_module::create(
///         "example",
///         OrganizationCustomModuleArgs::builder()
///             .custom_config(
///                 OrganizationCustomModuleCustomConfig::builder()
///                     .description(
///                         "The rotation period of the identified cryptokey resource exceeds 30 days.",
///                     )
///                     .predicate(
///                         OrganizationCustomModuleCustomConfigPredicate::builder()
///                             .expression(
///                                 "resource.rotationPeriod > duration(\"2592000s\")",
///                             )
///                             .build_struct(),
///                     )
///                     .recommendation("Set the rotation period to at most 30 days.")
///                     .resourceSelector(
///                         OrganizationCustomModuleCustomConfigResourceSelector::builder()
///                             .resourceTypes(vec!["cloudkms.googleapis.com/CryptoKey",])
///                             .build_struct(),
///                     )
///                     .severity("MEDIUM")
///                     .build_struct(),
///             )
///             .display_name("basic_custom_module")
///             .enablement_state("ENABLED")
///             .organization("123456789")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Scc Organization Custom Module Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization_custom_module::create(
///         "example",
///         OrganizationCustomModuleArgs::builder()
///             .custom_config(
///                 OrganizationCustomModuleCustomConfig::builder()
///                     .customOutput(
///                         OrganizationCustomModuleCustomConfigCustomOutput::builder()
///                             .properties(
///                                 vec![
///                                     OrganizationCustomModuleCustomConfigCustomOutputProperty::builder()
///                                     .name("duration")
///                                     .valueExpression(OrganizationCustomModuleCustomConfigCustomOutputPropertyValueExpression::builder()
///                                     .description("description of the expression")
///                                     .expression("resource.rotationPeriod")
///                                     .location("location of the expression")
///                                     .title("Purpose of the expression").build_struct())
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .description("Description of the custom module")
///                     .predicate(
///                         OrganizationCustomModuleCustomConfigPredicate::builder()
///                             .description("description of the expression")
///                             .expression(
///                                 "resource.rotationPeriod > duration(\"2592000s\")",
///                             )
///                             .location("location of the expression")
///                             .title("Purpose of the expression")
///                             .build_struct(),
///                     )
///                     .recommendation("Steps to resolve violation")
///                     .resourceSelector(
///                         OrganizationCustomModuleCustomConfigResourceSelector::builder()
///                             .resourceTypes(vec!["cloudkms.googleapis.com/CryptoKey",])
///                             .build_struct(),
///                     )
///                     .severity("LOW")
///                     .build_struct(),
///             )
///             .display_name("full_custom_module")
///             .enablement_state("ENABLED")
///             .organization("123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationCustomModule can be imported using any of these accepted formats:
///
/// * `organizations/{{organization}}/securityHealthAnalyticsSettings/customModules/{{name}}`
///
/// * `{{organization}}/{{name}}`
///
/// When using the `pulumi import` command, OrganizationCustomModule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/organizationCustomModule:OrganizationCustomModule default organizations/{{organization}}/securityHealthAnalyticsSettings/customModules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/organizationCustomModule:OrganizationCustomModule default {{organization}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_custom_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationCustomModuleArgs {
        /// The user specified custom configuration for the module.
        /// Structure is documented below.
        #[builder(into)]
        pub custom_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::securitycenter::OrganizationCustomModuleCustomConfig,
        >,
        /// The display name of the Security Health Analytics custom module. This
        /// display name becomes the finding category for all findings that are
        /// returned by this custom module. The display name must be between 1 and
        /// 128 characters, start with a lowercase letter, and contain alphanumeric
        /// characters or underscores only.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The enablement state of the custom module.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into)]
        pub enablement_state: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Numerical ID of the parent organization.
        #[builder(into)]
        pub organization: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationCustomModuleResult {
        /// If empty, indicates that the custom module was created in the organization, folder,
        /// or project in which you are viewing the custom module. Otherwise, ancestor_module
        /// specifies the organization or folder from which the custom module is inherited.
        pub ancestor_module: pulumi_gestalt_rust::Output<String>,
        /// The user specified custom configuration for the module.
        /// Structure is documented below.
        pub custom_config: pulumi_gestalt_rust::Output<
            super::super::types::securitycenter::OrganizationCustomModuleCustomConfig,
        >,
        /// The display name of the Security Health Analytics custom module. This
        /// display name becomes the finding category for all findings that are
        /// returned by this custom module. The display name must be between 1 and
        /// 128 characters, start with a lowercase letter, and contain alphanumeric
        /// characters or underscores only.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The enablement state of the custom module.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub enablement_state: pulumi_gestalt_rust::Output<String>,
        /// The editor that last updated the custom module.
        pub last_editor: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the custom module. Its format is "organizations/{org_id}/securityHealthAnalyticsSettings/customModules/{customModule}".
        /// The id {customModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Numerical ID of the parent organization.
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// The time at which the custom module was last updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and
        /// up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationCustomModuleArgs,
    ) -> OrganizationCustomModuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_config_binding = args.custom_config.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enablement_state_binding = args.enablement_state.get_output(context);
        let organization_binding = args.organization.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/organizationCustomModule:OrganizationCustomModule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customConfig".into(),
                    value: custom_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enablementState".into(),
                    value: enablement_state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organization".into(),
                    value: organization_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationCustomModuleResult {
            ancestor_module: o.get_field("ancestorModule"),
            custom_config: o.get_field("customConfig"),
            display_name: o.get_field("displayName"),
            enablement_state: o.get_field("enablementState"),
            last_editor: o.get_field("lastEditor"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            update_time: o.get_field("updateTime"),
        }
    }
}
