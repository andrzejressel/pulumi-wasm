/// Manages a Virtual Desktop Application Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let desktopapp = application_group::create(
///         "desktopapp",
///         ApplicationGroupArgs::builder()
///             .description("Acceptance Test: An application group")
///             .friendly_name("TestAppGroup")
///             .host_pool_id("${personalautomatic.id}")
///             .location("${example.location}")
///             .name("appgroupdesktop")
///             .resource_group_name("${example.name}")
///             .type_("Desktop")
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
/// Virtual Desktop Application Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/applicationGroup:ApplicationGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/applicationGroups/myapplicationgroup
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationGroupArgs {
        /// Option to set the display name for the default sessionDesktop desktop when `type` is set to `Desktop`. A value here is mandatory for connections to the desktop using the Windows 365 portal. Without it the connection will hang at 'Loading Client'.
        #[builder(into, default)]
        pub default_desktop_display_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Option to set a description for the Virtual Desktop Application Group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application Group.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource ID for a Virtual Desktop Host Pool to associate with the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        #[builder(into)]
        pub host_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location/region where the Virtual Desktop Application Group is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Virtual Desktop Application Group. Valid options are `RemoteApp` or `Desktop` application groups. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationGroupResult {
        /// Option to set the display name for the default sessionDesktop desktop when `type` is set to `Desktop`. A value here is mandatory for connections to the desktop using the Windows 365 portal. Without it the connection will hang at 'Loading Client'.
        pub default_desktop_display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application Group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application Group.
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource ID for a Virtual Desktop Host Pool to associate with the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        pub host_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The location/region where the Virtual Desktop Application Group is located. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Virtual Desktop Application Group. Valid options are `RemoteApp` or `Desktop` application groups. Changing this forces a new resource to be created.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationGroupArgs,
    ) -> ApplicationGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_desktop_display_name_binding = args
            .default_desktop_display_name
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let friendly_name_binding = args.friendly_name.get_output(context);
        let host_pool_id_binding = args.host_pool_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/applicationGroup:ApplicationGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultDesktopDisplayName".into(),
                    value: default_desktop_display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "friendlyName".into(),
                    value: friendly_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostPoolId".into(),
                    value: host_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationGroupResult {
            default_desktop_display_name: o.get_field("defaultDesktopDisplayName"),
            description: o.get_field("description"),
            friendly_name: o.get_field("friendlyName"),
            host_pool_id: o.get_field("hostPoolId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
