/// Manages an AppStream User Stack association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = stack::create(
///         "test",
///         StackArgs::builder().name("STACK NAME").build_struct(),
///     );
///     let testUser = user::create(
///         "testUser",
///         UserArgs::builder()
///             .authentication_type("USERPOOL")
///             .user_name("EMAIL")
///             .build_struct(),
///     );
///     let testUserStackAssociation = user_stack_association::create(
///         "testUserStackAssociation",
///         UserStackAssociationArgs::builder()
///             .authentication_type("${testUser.authenticationType}")
///             .stack_name("${test.name}")
///             .user_name("${testUser.userName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppStream User Stack Association using the `user_name`, `authentication_type`, and `stack_name`, separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appstream/userStackAssociation:UserStackAssociation example userName/auhtenticationType/stackName
/// ```
pub mod user_stack_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserStackAssociationArgs {
        /// Authentication type for the user.
        #[builder(into)]
        pub authentication_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether a welcome email is sent to a user after the user is created in the user pool.
        #[builder(into, default)]
        pub send_email_notification: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the stack that is associated with the user.
        #[builder(into)]
        pub stack_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Email address of the user who is associated with the stack.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserStackAssociationResult {
        /// Authentication type for the user.
        pub authentication_type: pulumi_wasm_rust::Output<String>,
        /// Whether a welcome email is sent to a user after the user is created in the user pool.
        pub send_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the stack that is associated with the user.
        pub stack_name: pulumi_wasm_rust::Output<String>,
        /// Email address of the user who is associated with the stack.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserStackAssociationArgs,
    ) -> UserStackAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let send_email_notification_binding = args
            .send_email_notification
            .get_output(context)
            .get_inner();
        let stack_name_binding = args.stack_name.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/userStackAssociation:UserStackAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "sendEmailNotification".into(),
                    value: &send_email_notification_binding,
                },
                register_interface::ObjectField {
                    name: "stackName".into(),
                    value: &stack_name_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationType".into(),
                },
                register_interface::ResultField {
                    name: "sendEmailNotification".into(),
                },
                register_interface::ResultField {
                    name: "stackName".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserStackAssociationResult {
            authentication_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationType").unwrap(),
            ),
            send_email_notification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendEmailNotification").unwrap(),
            ),
            stack_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackName").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
