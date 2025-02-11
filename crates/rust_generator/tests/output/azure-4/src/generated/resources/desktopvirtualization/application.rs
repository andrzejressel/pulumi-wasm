/// Manages a Virtual Desktop Application.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let chrome = application::create(
///         "chrome",
///         ApplicationArgs::builder()
///             .application_group_id("${remoteapp.id}")
///             .command_line_argument_policy("DoNotAllow")
///             .command_line_arguments("--incognito")
///             .description("Chromium based web browser")
///             .friendly_name("Google Chrome")
///             .icon_index(0)
///             .icon_path("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
///             .name("googlechrome")
///             .path("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
///             .show_in_portal(false)
///             .build_struct(),
///     );
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("rg-example-virtualdesktop")
///             .build_struct(),
///     );
///     let personalautomatic = host_pool::create(
///         "personalautomatic",
///         HostPoolArgs::builder()
///             .load_balancer_type("BreadthFirst")
///             .location("${example.location}")
///             .name("personalautomatic")
///             .personal_desktop_assignment_type("Automatic")
///             .resource_group_name("${example.name}")
///             .type_("Personal")
///             .build_struct(),
///     );
///     let pooledbreadthfirst = host_pool::create(
///         "pooledbreadthfirst",
///         HostPoolArgs::builder()
///             .load_balancer_type("BreadthFirst")
///             .location("${example.location}")
///             .name("pooledbreadthfirst")
///             .resource_group_name("${example.name}")
///             .type_("Pooled")
///             .build_struct(),
///     );
///     let remoteapp = application_group::create(
///         "remoteapp",
///         ApplicationGroupArgs::builder()
///             .description("Acceptance Test: An application group")
///             .friendly_name("TestAppGroup")
///             .host_pool_id("${pooledbreadthfirst.id}")
///             .location("${example.location}")
///             .name("acctag")
///             .resource_group_name("${example.name}")
///             .type_("RemoteApp")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Desktop Application can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/application:Application example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/applicationGroups/myapplicationgroup/applications/myapplication
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// Resource ID for a Virtual Desktop Application Group to associate with the Virtual Desktop Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all. Possible values include: `DoNotAllow`, `Allow`, `Require`.
        #[builder(into)]
        pub command_line_argument_policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Command Line Arguments for Virtual Desktop Application.
        #[builder(into, default)]
        pub command_line_arguments: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The index of the icon you wish to use.
        #[builder(into, default)]
        pub icon_index: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the path for an icon which will be used for this Virtual Desktop Application.
        #[builder(into, default)]
        pub icon_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Virtual Desktop Application. Changing the name forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The file path location of the app on the Virtual Desktop OS.
        #[builder(into)]
        pub path: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether to show the RemoteApp program in the RD Web Access server.
        #[builder(into, default)]
        pub show_in_portal: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Resource ID for a Virtual Desktop Application Group to associate with the Virtual Desktop Application. Changing this forces a new resource to be created.
        pub application_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all. Possible values include: `DoNotAllow`, `Allow`, `Require`.
        pub command_line_argument_policy: pulumi_gestalt_rust::Output<String>,
        /// Command Line Arguments for Virtual Desktop Application.
        pub command_line_arguments: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application.
        pub friendly_name: pulumi_gestalt_rust::Output<String>,
        /// The index of the icon you wish to use.
        pub icon_index: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the path for an icon which will be used for this Virtual Desktop Application.
        pub icon_path: pulumi_gestalt_rust::Output<String>,
        /// The name of the Virtual Desktop Application. Changing the name forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The file path location of the app on the Virtual Desktop OS.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether to show the RemoteApp program in the RD Web Access server.
        pub show_in_portal: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_group_id_binding = args.application_group_id.get_output(context);
        let command_line_argument_policy_binding = args
            .command_line_argument_policy
            .get_output(context);
        let command_line_arguments_binding = args
            .command_line_arguments
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let friendly_name_binding = args.friendly_name.get_output(context);
        let icon_index_binding = args.icon_index.get_output(context);
        let icon_path_binding = args.icon_path.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_binding = args.path.get_output(context);
        let show_in_portal_binding = args.show_in_portal.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationGroupId".into(),
                    value: &application_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commandLineArgumentPolicy".into(),
                    value: &command_line_argument_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commandLineArguments".into(),
                    value: &command_line_arguments_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iconIndex".into(),
                    value: &icon_index_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iconPath".into(),
                    value: &icon_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "showInPortal".into(),
                    value: &show_in_portal_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            application_group_id: o.get_field("applicationGroupId"),
            command_line_argument_policy: o.get_field("commandLineArgumentPolicy"),
            command_line_arguments: o.get_field("commandLineArguments"),
            description: o.get_field("description"),
            friendly_name: o.get_field("friendlyName"),
            icon_index: o.get_field("iconIndex"),
            icon_path: o.get_field("iconPath"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            show_in_portal: o.get_field("showInPortal"),
        }
    }
}
