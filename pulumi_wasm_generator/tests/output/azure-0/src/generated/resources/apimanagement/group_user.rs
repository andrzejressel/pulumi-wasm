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
pub mod group_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupUserArgs {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the API Management User which should be assigned to this API Management Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GroupUserResult {
        /// The name of the API Management Service. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The Name of the API Management Group within the API Management Service. Changing this forces a new resource to be created.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the API Management User which should be assigned to this API Management Group. Changing this forces a new resource to be created.
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupUserArgs) -> GroupUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let group_name_binding = args.group_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/groupUser:GroupUser".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupUserResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}
