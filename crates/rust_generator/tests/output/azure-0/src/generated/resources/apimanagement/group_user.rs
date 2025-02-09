/// Manages an API Management User Assignment to a Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGroupUser:
///     type: azure:apimanagement:GroupUser
///     name: example
///     properties:
///       userId: ${example.id}
///       groupName: example-group
///       resourceGroupName: ${example.resourceGroupName}
///       apiManagementName: ${example.apiManagementName}
/// variables:
///   example:
///     fn::invoke:
///       function: azure:apimanagement:getUser
///       arguments:
///         userId: my-user
///         apiManagementName: example-apim
///         resourceGroupName: search-service
/// ```
///
/// ## Import
///
/// API Management Group Users can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/groupUser:GroupUser example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/service1/groups/groupId/users/user123
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupUserArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the API Management User which should be assigned to this API Management Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupUserResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the API Management User which should be assigned to this API Management Group. Changing this forces a new resource to be created.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupUserArgs,
    ) -> GroupUserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let group_name_binding = args.group_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/groupUser:GroupUser".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupUserResult {
            api_management_name: o.get_field("apiManagementName"),
            group_name: o.get_field("groupName"),
            resource_group_name: o.get_field("resourceGroupName"),
            user_id: o.get_field("userId"),
        }
    }
}
