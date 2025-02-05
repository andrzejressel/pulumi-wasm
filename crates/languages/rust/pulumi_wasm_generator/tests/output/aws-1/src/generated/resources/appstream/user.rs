/// Provides an AppStream user.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .authentication_type("USERPOOL")
///             .first_name("FIRST NAME")
///             .last_name("LAST NAME")
///             .user_name("EMAIL")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_user` using the `user_name` and `authentication_type` separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appstream/user:User example UserName/AuthenticationType
/// ```
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Authentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
        #[builder(into)]
        pub authentication_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether the user in the user pool is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// First name, or given name, of the user.
        #[builder(into, default)]
        pub first_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Last name, or surname, of the user.
        #[builder(into, default)]
        pub last_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Send an email notification.
        #[builder(into, default)]
        pub send_email_notification: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Email address of the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// ARN of the appstream user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Authentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
        pub authentication_type: pulumi_wasm_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when the user was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Whether the user in the user pool is enabled.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// First name, or given name, of the user.
        pub first_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Last name, or surname, of the user.
        pub last_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Send an email notification.
        pub send_email_notification: pulumi_wasm_rust::Output<Option<bool>>,
        /// Email address of the user.
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
        args: UserArgs,
    ) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_type_binding = args
            .authentication_type
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let first_name_binding = args.first_name.get_output(context).get_inner();
        let last_name_binding = args.last_name.get_output(context).get_inner();
        let send_email_notification_binding = args
            .send_email_notification
            .get_output(context)
            .get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
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
                    name: "sendEmailNotification".into(),
                    value: &send_email_notification_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            authentication_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationType"),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            first_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firstName"),
            ),
            last_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastName"),
            ),
            send_email_notification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sendEmailNotification"),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
