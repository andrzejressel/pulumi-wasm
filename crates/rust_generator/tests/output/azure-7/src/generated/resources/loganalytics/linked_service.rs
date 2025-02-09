/// Manages a Log Analytics Linked Service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: resourcegroup-01
///       location: West Europe
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: automation-01
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///       tags:
///         environment: development
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: workspace-01
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///       retentionInDays: 30
///   exampleLinkedService:
///     type: azure:loganalytics:LinkedService
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       workspaceId: ${exampleAnalyticsWorkspace.id}
///       readAccessId: ${exampleAccount.id}
/// ```
///
/// ## Import
///
/// Log Analytics Workspaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/linkedService:LinkedService example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.OperationalInsights/workspaces/workspace1/linkedServices/Automation
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linked_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinkedServiceArgs {
        /// The ID of the readable Resource that will be linked to the workspace. This should be used for linking to an Automation Account resource.
        #[builder(into, default)]
        pub read_access_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Log Analytics Linked Service is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Log Analytics Workspace that will contain the Log Analytics Linked Service resource.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the writable Resource that will be linked to the workspace. This should be used for linking to a Log Analytics Cluster resource.
        ///
        /// > **NOTE:** You must define at least one of the above access resource id attributes (e.g. `read_access_id` or `write_access_id`).
        #[builder(into, default)]
        pub write_access_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinkedServiceResult {
        /// The generated name of the Linked Service. The format for this attribute is always `<workspace name>/<linked service type>`(e.g. `workspace1/Automation` or `workspace1/Cluster`)
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the readable Resource that will be linked to the workspace. This should be used for linking to an Automation Account resource.
        pub read_access_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which the Log Analytics Linked Service is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Log Analytics Workspace that will contain the Log Analytics Linked Service resource.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the writable Resource that will be linked to the workspace. This should be used for linking to a Log Analytics Cluster resource.
        ///
        /// > **NOTE:** You must define at least one of the above access resource id attributes (e.g. `read_access_id` or `write_access_id`).
        pub write_access_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinkedServiceArgs,
    ) -> LinkedServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let read_access_id_binding = args.read_access_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let write_access_id_binding = args.write_access_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:loganalytics/linkedService:LinkedService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readAccessId".into(),
                    value: read_access_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "writeAccessId".into(),
                    value: write_access_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinkedServiceResult {
            name: o.get_field("name"),
            read_access_id: o.get_field("readAccessId"),
            resource_group_name: o.get_field("resourceGroupName"),
            workspace_id: o.get_field("workspaceId"),
            write_access_id: o.get_field("writeAccessId"),
        }
    }
}
