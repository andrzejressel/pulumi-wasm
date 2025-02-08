/// Represents an instance of a Security Health Analytics custom module, including
/// its full module name, display name, enablement state, and last updated time.
/// You can create a custom module at the organization, folder, or project level.
/// Custom modules that you create at the organization or folder level are inherited
/// by the child folders and projects.
///
///
/// To get more information about FolderSecurityHealthAnalyticsCustomModule, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/security-center-management/rest/v1/folders.locations.securityHealthAnalyticsCustomModules)
/// * How-to Guides
///     * [Overview of custom modules for Security Health Analytics](https://cloud.google.com/security-command-center/docs/custom-modules-sha-overview)
///
/// ## Example Usage
///
/// ### Scc Management Folder Security Health Analytics Custom Module Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = management_folder_security_health_analytics_custom_module::create(
///         "example",
///         ManagementFolderSecurityHealthAnalyticsCustomModuleArgs::builder()
///             .custom_config(
///                 ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfig::builder()
///                     .description(
///                         "The rotation period of the identified cryptokey resource exceeds 30 days.",
///                     )
///                     .predicate(
///                         ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigPredicate::builder()
///                             .expression(
///                                 "resource.rotationPeriod > duration(\"2592000s\")",
///                             )
///                             .build_struct(),
///                     )
///                     .recommendation("Set the rotation period to at most 30 days.")
///                     .resourceSelector(
///                         ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigResourceSelector::builder()
///                             .resourceTypes(vec!["cloudkms.googleapis.com/CryptoKey",])
///                             .build_struct(),
///                     )
///                     .severity("MEDIUM")
///                     .build_struct(),
///             )
///             .display_name("basic_custom_module")
///             .enablement_state("ENABLED")
///             .folder("${folder.folderId}")
///             .location("global")
///             .build_struct(),
///     );
///     let folder = folder::create(
///         "folder",
///         FolderArgs::builder()
///             .deletion_protection(false)
///             .display_name("folder-name")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Scc Management Folder Security Health Analytics Custom Module Full
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = management_folder_security_health_analytics_custom_module::create(
///         "example",
///         ManagementFolderSecurityHealthAnalyticsCustomModuleArgs::builder()
///             .custom_config(
///                 ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfig::builder()
///                     .customOutput(
///                         ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutput::builder()
///                             .properties(
///                                 vec![
///                                     ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutputProperty::builder()
///                                     .name("duration")
///                                     .valueExpression(ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutputPropertyValueExpression::builder()
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
///                         ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigPredicate::builder()
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
///                         ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigResourceSelector::builder()
///                             .resourceTypes(vec!["cloudkms.googleapis.com/CryptoKey",])
///                             .build_struct(),
///                     )
///                     .severity("LOW")
///                     .build_struct(),
///             )
///             .display_name("full_custom_module")
///             .enablement_state("ENABLED")
///             .folder("${folder.folderId}")
///             .location("global")
///             .build_struct(),
///     );
///     let folder = folder::create(
///         "folder",
///         FolderArgs::builder()
///             .deletion_protection(false)
///             .display_name("folder-name")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FolderSecurityHealthAnalyticsCustomModule can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/locations/{{location}}/securityHealthAnalyticsCustomModules/{{name}}`
///
/// * `{{folder}}/{{location}}/{{name}}`
///
/// When using the `pulumi import` command, FolderSecurityHealthAnalyticsCustomModule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/managementFolderSecurityHealthAnalyticsCustomModule:ManagementFolderSecurityHealthAnalyticsCustomModule default folders/{{folder}}/locations/{{location}}/securityHealthAnalyticsCustomModules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/managementFolderSecurityHealthAnalyticsCustomModule:ManagementFolderSecurityHealthAnalyticsCustomModule default {{folder}}/{{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod management_folder_security_health_analytics_custom_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagementFolderSecurityHealthAnalyticsCustomModuleArgs {
        /// The user specified custom configuration for the module.
        /// Structure is documented below.
        #[builder(into, default)]
        pub custom_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::securitycenter::ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfig,
            >,
        >,
        /// The display name of the Security Health Analytics custom module. This
        /// display name becomes the finding category for all findings that are
        /// returned by this custom module. The display name must be between 1 and
        /// 128 characters, start with a lowercase letter, and contain alphanumeric
        /// characters or underscores only.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The enablement state of the custom module.
        /// Possible values are: `ENABLED`, `DISABLED`.
        #[builder(into, default)]
        pub enablement_state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Numerical ID of the parent folder.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Location ID of the parent organization. If not provided, 'global' will be used as the default location.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagementFolderSecurityHealthAnalyticsCustomModuleResult {
        /// If empty, indicates that the custom module was created in the organization, folder,
        /// or project in which you are viewing the custom module. Otherwise, ancestor_module
        /// specifies the organization or folder from which the custom module is inherited.
        pub ancestor_module: pulumi_gestalt_rust::Output<String>,
        /// The user specified custom configuration for the module.
        /// Structure is documented below.
        pub custom_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::securitycenter::ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfig,
            >,
        >,
        /// The display name of the Security Health Analytics custom module. This
        /// display name becomes the finding category for all findings that are
        /// returned by this custom module. The display name must be between 1 and
        /// 128 characters, start with a lowercase letter, and contain alphanumeric
        /// characters or underscores only.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The enablement state of the custom module.
        /// Possible values are: `ENABLED`, `DISABLED`.
        pub enablement_state: pulumi_gestalt_rust::Output<Option<String>>,
        /// Numerical ID of the parent folder.
        ///
        ///
        /// - - -
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The editor that last updated the custom module.
        pub last_editor: pulumi_gestalt_rust::Output<String>,
        /// Location ID of the parent organization. If not provided, 'global' will be used as the default location.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the custom module. Its format is "folders/{folder}/locations/{location}/securityHealthAnalyticsCustomModules/{securityHealthAnalyticsCustomModule}".
        /// The id {securityHealthAnalyticsCustomModule} is server-generated and is not user settable. It will be a numeric id containing 1-20 digits.
        pub name: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagementFolderSecurityHealthAnalyticsCustomModuleArgs,
    ) -> ManagementFolderSecurityHealthAnalyticsCustomModuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_config_binding = args.custom_config.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enablement_state_binding = args
            .enablement_state
            .get_output(context)
            .get_inner();
        let folder_binding = args.folder.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/managementFolderSecurityHealthAnalyticsCustomModule:ManagementFolderSecurityHealthAnalyticsCustomModule"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customConfig".into(),
                    value: &custom_config_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enablementState".into(),
                    value: &enablement_state_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagementFolderSecurityHealthAnalyticsCustomModuleResult {
            ancestor_module: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ancestorModule"),
            ),
            custom_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customConfig"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enablement_state: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enablementState"),
            ),
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            last_editor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastEditor"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
