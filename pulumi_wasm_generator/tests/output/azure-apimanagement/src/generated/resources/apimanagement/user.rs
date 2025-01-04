/// Manages an API Management User.
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
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The name of the API Management Service in which the User should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The kind of confirmation email which will be sent to this user. Possible values are `invite` and `signup`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub confirmation: pulumi_wasm_rust::Output<Option<String>>,
        /// The email address associated with this user.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// The first name for this user.
        #[builder(into)]
        pub first_name: pulumi_wasm_rust::Output<String>,
        /// The last name for this user.
        #[builder(into)]
        pub last_name: pulumi_wasm_rust::Output<String>,
        /// A note about this user.
        #[builder(into, default)]
        pub note: pulumi_wasm_rust::Output<Option<String>>,
        /// The password associated with this user.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The state of this user. Possible values are `active`, `blocked` and `pending`.
        ///
        /// > **NOTE:** the State can be changed from Pending > Active/Blocked but not from Active/Blocked > Pending.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// The Identifier for this User, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// The name of the API Management Service in which the User should be created. Changing this forces a new resource to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The kind of confirmation email which will be sent to this user. Possible values are `invite` and `signup`. Changing this forces a new resource to be created.
        pub confirmation: pulumi_wasm_rust::Output<Option<String>>,
        /// The email address associated with this user.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The first name for this user.
        pub first_name: pulumi_wasm_rust::Output<String>,
        /// The last name for this user.
        pub last_name: pulumi_wasm_rust::Output<String>,
        /// A note about this user.
        pub note: pulumi_wasm_rust::Output<Option<String>>,
        /// The password associated with this user.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the API Management Service exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The state of this user. Possible values are `active`, `blocked` and `pending`.
        ///
        /// > **NOTE:** the State can be changed from Pending > Active/Blocked but not from Active/Blocked > Pending.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The Identifier for this User, which must be unique within the API Management Service. Changing this forces a new resource to be created.
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserArgs) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let confirmation_binding = args.confirmation.get_inner();
        let email_binding = args.email.get_inner();
        let first_name_binding = args.first_name.get_inner();
        let last_name_binding = args.last_name.get_inner();
        let note_binding = args.note.get_inner();
        let password_binding = args.password.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let state_binding = args.state.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/user:User".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "confirmation".into(),
                    value: &confirmation_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding,
                },
                register_interface::ObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding,
                },
                register_interface::ObjectField {
                    name: "note".into(),
                    value: &note_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
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
                    name: "confirmation".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "firstName".into(),
                },
                register_interface::ResultField {
                    name: "lastName".into(),
                },
                register_interface::ResultField {
                    name: "note".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
        UserResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            confirmation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confirmation").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            first_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("firstName").unwrap(),
            ),
            last_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastName").unwrap(),
            ),
            note: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("note").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}
