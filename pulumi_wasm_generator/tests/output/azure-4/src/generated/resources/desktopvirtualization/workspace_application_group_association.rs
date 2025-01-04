/// Manages a Virtual Desktop Workspace Application Group Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("rg-example-virtualdesktop")
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
///             .host_pool_id("${pooledbreadthfirst.id}")
///             .location("${example.location}")
///             .name("remoteapp")
///             .resource_group_name("${example.name}")
///             .type_("RemoteApp")
///             .build_struct(),
///     );
///     let workspace = workspace::create(
///         "workspace",
///         WorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("workspace")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let workspaceremoteapp = workspace_application_group_association::create(
///         "workspaceremoteapp",
///         WorkspaceApplicationGroupAssociationArgs::builder()
///             .application_group_id("${remoteapp.id}")
///             .workspace_id("${workspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Associations between Virtual Desktop Workspaces and Virtual Desktop Application Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/workspaceApplicationGroupAssociation:WorkspaceApplicationGroupAssociation association1 "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/workspaces/myworkspace|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/applicationGroups/myapplicationgroup"
/// ```
///
pub mod workspace_application_group_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceApplicationGroupAssociationArgs {
        /// The resource ID for the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub application_group_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID for the Virtual Desktop Workspace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkspaceApplicationGroupAssociationResult {
        /// The resource ID for the Virtual Desktop Application Group. Changing this forces a new resource to be created.
        pub application_group_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID for the Virtual Desktop Workspace. Changing this forces a new resource to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WorkspaceApplicationGroupAssociationArgs,
    ) -> WorkspaceApplicationGroupAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_group_id_binding = args.application_group_id.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/workspaceApplicationGroupAssociation:WorkspaceApplicationGroupAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationGroupId".into(),
                    value: &application_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceApplicationGroupAssociationResult {
            application_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationGroupId").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
