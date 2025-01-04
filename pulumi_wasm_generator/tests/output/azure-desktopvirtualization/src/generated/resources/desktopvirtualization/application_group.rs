/// Manages a Virtual Desktop Application Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod application_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationGroupArgs {
        /// Option to set the display name for the default sessionDesktop desktop when `type` is set to `Desktop`. A value here is mandatory for connections to the desktop using the Windows 365 portal. Without it the connection will hang at 'Loading Client'.
        #[builder(into, default)]
        pub default_desktop_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application Group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application Group.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource ID for a Virtual Desktop Host Pool to associate with the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        #[builder(into)]
        pub host_pool_id: pulumi_wasm_rust::Output<String>,
        /// The location/region where the Virtual Desktop Application Group is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Virtual Desktop Application Group. Valid options are `RemoteApp` or `Desktop` application groups. Changing this forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationGroupResult {
        /// Option to set the display name for the default sessionDesktop desktop when `type` is set to `Desktop`. A value here is mandatory for connections to the desktop using the Windows 365 portal. Without it the connection will hang at 'Loading Client'.
        pub default_desktop_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a description for the Virtual Desktop Application Group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to set a friendly name for the Virtual Desktop Application Group.
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource ID for a Virtual Desktop Host Pool to associate with the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        pub host_pool_id: pulumi_wasm_rust::Output<String>,
        /// The location/region where the Virtual Desktop Application Group is located. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Virtual Desktop Application Group. Changing the name forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Virtual Desktop Application Group. Valid options are `RemoteApp` or `Desktop` application groups. Changing this forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationGroupArgs) -> ApplicationGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_desktop_display_name_binding = args
            .default_desktop_display_name
            .get_inner();
        let description_binding = args.description.get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let host_pool_id_binding = args.host_pool_id.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/applicationGroup:ApplicationGroup"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultDesktopDisplayName".into(),
                    value: &default_desktop_display_name_binding,
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
                    name: "hostPoolId".into(),
                    value: &host_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultDesktopDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "hostPoolId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationGroupResult {
            default_desktop_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultDesktopDisplayName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            host_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostPoolId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
