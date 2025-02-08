/// Mute Findings is a volume management feature in Security Command Center
/// that lets you manually or programmatically hide irrelevant findings,
/// and create filters to automatically silence existing and future
/// findings based on criteria you specify.
///
///
/// To get more information about ProjectMuteConfig, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/projects.muteConfigs)
///
/// ## Example Usage
///
/// ### Scc V2 Project Mute Config Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = v_2_project_mute_config::create(
///         "default",
///         V2ProjectMuteConfigArgs::builder()
///             .description(
///                 "My custom Cloud Security Command Center Finding Project mute Configuration",
///             )
///             .filter("severity = \"HIGH\"")
///             .location("global")
///             .mute_config_id("my-config")
///             .project("my-project-name")
///             .type_("STATIC")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ProjectMuteConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/muteConfigs/{{mute_config_id}}`
///
/// * `{{project}}/{{location}}/{{mute_config_id}}`
///
/// * `{{location}}/{{mute_config_id}}`
///
/// When using the `pulumi import` command, ProjectMuteConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectMuteConfig:V2ProjectMuteConfig default projects/{{project}}/locations/{{location}}/muteConfigs/{{mute_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectMuteConfig:V2ProjectMuteConfig default {{project}}/{{location}}/{{mute_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2ProjectMuteConfig:V2ProjectMuteConfig default {{location}}/{{mute_config_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_project_mute_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2ProjectMuteConfigArgs {
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
        /// location Id is provided by project. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub mute_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the mute config.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct V2ProjectMuteConfigResult {
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
        /// location Id is provided by project. If not provided, Use global as default.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Email address of the user who last edited the mute config. This
        /// field is set by the server and will be ignored if provided on
        /// config creation or update.
        pub most_recent_editor: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier provided by the client within the parent scope.
        ///
        ///
        /// - - -
        pub mute_config_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the mute config. Its format is
        /// projects/{project}/locations/global/muteConfigs/{configId},
        /// folders/{folder}/locations/global/muteConfigs/{configId},
        /// or organizations/{organization}/locations/global/muteConfigs/{configId}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The type of the mute config.
        pub type_: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2ProjectMuteConfigArgs,
    ) -> V2ProjectMuteConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mute_config_id_binding = args.mute_config_id.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2ProjectMuteConfig:V2ProjectMuteConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "muteConfigId".into(),
                    value: &mute_config_id_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2ProjectMuteConfigResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            most_recent_editor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecentEditor"),
            ),
            mute_config_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("muteConfigId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
