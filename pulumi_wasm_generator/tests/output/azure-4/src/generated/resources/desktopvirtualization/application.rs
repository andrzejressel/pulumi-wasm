/// Manages a Virtual Desktop Application.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// Resource ID for a Virtual Desktop Application Group to associate with the Virtual Desktop Application. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all. Possible values include: `DoNotAllow`, `Allow`, `Require`.
        #[builder(into)]
        pub command_line_argument_policy: pulumi_wasm_rust::InputOrOutput<String>,
        /// Command Line Arguments for Virtual Desktop Application.
        #[builder(into, default)]
        pub command_line_arguments: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The index of the icon you wish to use.
        #[builder(into, default)]
        pub icon_index: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies the path for an icon which will be used for this Virtual Desktop Application.
        #[builder(into, default)]
        pub icon_path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Virtual Desktop Application. Changing the name forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The file path location of the app on the Virtual Desktop OS.
        #[builder(into)]
        pub path: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies whether to show the RemoteApp program in the RD Web Access server.
        #[builder(into, default)]
        pub show_in_portal: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Resource ID for a Virtual Desktop Application Group to associate with the Virtual Desktop Application. Changing this forces a new resource to be created.
        pub application_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies whether this published application can be launched with command line arguments provided by the client, command line arguments specified at publish time, or no command line arguments at all. Possible values include: `DoNotAllow`, `Allow`, `Require`.
        pub command_line_argument_policy: pulumi_wasm_rust::Output<String>,
        /// Command Line Arguments for Virtual Desktop Application.
        pub command_line_arguments: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application.
        pub friendly_name: pulumi_wasm_rust::Output<String>,
        /// The index of the icon you wish to use.
        pub icon_index: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the path for an icon which will be used for this Virtual Desktop Application.
        pub icon_path: pulumi_wasm_rust::Output<String>,
        /// The name of the Virtual Desktop Application. Changing the name forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The file path location of the app on the Virtual Desktop OS.
        pub path: pulumi_wasm_rust::Output<String>,
        /// Specifies whether to show the RemoteApp program in the RD Web Access server.
        pub show_in_portal: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_group_id_binding = args
            .application_group_id
            .get_output(context)
            .get_inner();
        let command_line_argument_policy_binding = args
            .command_line_argument_policy
            .get_output(context)
            .get_inner();
        let command_line_arguments_binding = args
            .command_line_arguments
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let friendly_name_binding = args.friendly_name.get_output(context).get_inner();
        let icon_index_binding = args.icon_index.get_output(context).get_inner();
        let icon_path_binding = args.icon_path.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let show_in_portal_binding = args.show_in_portal.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationGroupId".into(),
                    value: &application_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "commandLineArgumentPolicy".into(),
                    value: &command_line_argument_policy_binding,
                },
                register_interface::ObjectField {
                    name: "commandLineArguments".into(),
                    value: &command_line_arguments_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "iconIndex".into(),
                    value: &icon_index_binding,
                },
                register_interface::ObjectField {
                    name: "iconPath".into(),
                    value: &icon_path_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "showInPortal".into(),
                    value: &show_in_portal_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "commandLineArgumentPolicy".into(),
                },
                register_interface::ResultField {
                    name: "commandLineArguments".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "iconIndex".into(),
                },
                register_interface::ResultField {
                    name: "iconPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "showInPortal".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            application_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationGroupId").unwrap(),
            ),
            command_line_argument_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commandLineArgumentPolicy").unwrap(),
            ),
            command_line_arguments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commandLineArguments").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            icon_index: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iconIndex").unwrap(),
            ),
            icon_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iconPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            show_in_portal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("showInPortal").unwrap(),
            ),
        }
    }
}
