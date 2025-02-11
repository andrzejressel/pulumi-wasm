/// Manages an API Management User.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@exmaple.com")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
///     let exampleUser = user::create(
///         "exampleUser",
///         UserArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .email("user@example.com")
///             .first_name("Example")
///             .last_name("User")
///             .resource_group_name("${example.name}")
///             .state("active")
///             .user_id("5931a75ae4bbd512288c680b")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Users can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/user:User example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ApiManagement/service/instance1/users/abc123
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The name of the API Management Service in which the User should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The kind of confirmation email which will be sent to this user. Possible values are `invite` and `signup`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub confirmation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The email address associated with this user.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The first name for this user.
        #[builder(into)]
        pub first_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The last name for this user.
        #[builder(into)]
        pub last_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A note about this user.
        #[builder(into, default)]
        pub note: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password associated with this user.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The state of this user. Possible values are `active`, `blocked` and `pending`.
        ///
        /// > **NOTE:** the State can be changed from Pending > Active/Blocked but not from Active/Blocked > Pending.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Identifier for this User, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// The name of the API Management Service in which the User should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The kind of confirmation email which will be sent to this user. Possible values are `invite` and `signup`. Changing this forces a new resource to be created.
        pub confirmation: pulumi_gestalt_rust::Output<Option<String>>,
        /// The email address associated with this user.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The first name for this user.
        pub first_name: pulumi_gestalt_rust::Output<String>,
        /// The last name for this user.
        pub last_name: pulumi_gestalt_rust::Output<String>,
        /// A note about this user.
        pub note: pulumi_gestalt_rust::Output<Option<String>>,
        /// The password associated with this user.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The state of this user. Possible values are `active`, `blocked` and `pending`.
        ///
        /// > **NOTE:** the State can be changed from Pending > Active/Blocked but not from Active/Blocked > Pending.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The Identifier for this User, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let confirmation_binding = args.confirmation.get_output(context);
        let email_binding = args.email.get_output(context);
        let first_name_binding = args.first_name.get_output(context);
        let last_name_binding = args.last_name.get_output(context);
        let note_binding = args.note.get_output(context);
        let password_binding = args.password.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let state_binding = args.state.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:apimanagement/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "confirmation".into(),
                    value: &confirmation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: &email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "note".into(),
                    value: &note_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: &state_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: &user_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            api_management_name: o.get_field("apiManagementName"),
            confirmation: o.get_field("confirmation"),
            email: o.get_field("email"),
            first_name: o.get_field("firstName"),
            last_name: o.get_field("lastName"),
            note: o.get_field("note"),
            password: o.get_field("password"),
            resource_group_name: o.get_field("resourceGroupName"),
            state: o.get_field("state"),
            user_id: o.get_field("userId"),
        }
    }
}
