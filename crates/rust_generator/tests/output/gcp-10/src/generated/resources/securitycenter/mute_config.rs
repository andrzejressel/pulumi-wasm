/// Mute Findings is a volume management feature in Security Command Center
/// that lets you manually or programmatically hide irrelevant findings,
/// and create filters to automatically silence existing and future
/// findings based on criteria you specify.
///
///
/// To get more information about MuteConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/organizations.muteConfigs)
///
/// ## Example Usage
///
/// ### Scc Mute Config
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = mute_config::create(
///         "default",
///         MuteConfigArgs::builder()
///             .description("My Mute Config")
///             .filter("category: \"OS_VULNERABILITY\"")
///             .mute_config_id("my-config")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MuteConfig can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, MuteConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/muteConfig:MuteConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mute_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MuteConfigArgs {
        /// A description of the mute config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique identifier provided by the client within the parent scope.
        #[builder(into)]
        pub mute_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Resource name of the new mute configs's parent. Its format is
        /// "organizations/[organization_id]", "folders/[folder_id]", or
        /// "projects/[project_id]".
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MuteConfigResult {
        /// The time at which the mute config was created. This field is set by
        /// the server and will be ignored if provided on config creation.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the mute config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An expression that defines the filter to apply across create/update
        /// events of findings. While creating a filter string, be mindful of
        /// the scope in which the mute configuration is being created. E.g.,
        /// If a filter contains project = X but is created under the
        /// project = Y scope, it might not match any findings.
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// Email address of the user who last edited the mute config. This
        /// field is set by the server and will be ignored if provided on
        /// config creation or update.
        pub most_recent_editor: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        pub mute_config_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the mute config. Its format is
        /// organizations/{organization}/muteConfigs/{configId},
        /// folders/{folder}/muteConfigs/{configId},
        /// or projects/{project}/muteConfigs/{configId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Resource name of the new mute configs's parent. Its format is
        /// "organizations/[organization_id]", "folders/[folder_id]", or
        /// "projects/[project_id]".
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Output only. The most recent time at which the mute config was
        /// updated. This field is set by the server and will be ignored if
        /// provided on config creation or update.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MuteConfigArgs,
    ) -> MuteConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let mute_config_id_binding = args.mute_config_id.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/muteConfig:MuteConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "muteConfigId".into(),
                    value: mute_config_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MuteConfigResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            filter: o.get_field("filter"),
            most_recent_editor: o.get_field("mostRecentEditor"),
            mute_config_id: o.get_field("muteConfigId"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            update_time: o.get_field("updateTime"),
        }
    }
}
