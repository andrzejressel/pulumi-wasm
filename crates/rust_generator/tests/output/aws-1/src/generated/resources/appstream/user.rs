/// Provides an AppStream user.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Authentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
        #[builder(into)]
        pub authentication_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the user in the user pool is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// First name, or given name, of the user.
        #[builder(into, default)]
        pub first_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Last name, or surname, of the user.
        #[builder(into, default)]
        pub last_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Send an email notification.
        #[builder(into, default)]
        pub send_email_notification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Email address of the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// ARN of the appstream user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Authentication type for the user. You must specify USERPOOL. Valid values: `API`, `SAML`, `USERPOOL`
        pub authentication_type: pulumi_gestalt_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when the user was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Whether the user in the user pool is enabled.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// First name, or given name, of the user.
        pub first_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Last name, or surname, of the user.
        pub last_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Send an email notification.
        pub send_email_notification: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Email address of the user.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_gestalt_rust::Output<String>,
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
        let authentication_type_binding = args.authentication_type.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let first_name_binding = args.first_name.get_output(context);
        let last_name_binding = args.last_name.get_output(context);
        let send_email_notification_binding = args
            .send_email_notification
            .get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appstream/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationType".into(),
                    value: authentication_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "firstName".into(),
                    value: first_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lastName".into(),
                    value: last_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sendEmailNotification".into(),
                    value: send_email_notification_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            arn: o.get_field("arn"),
            authentication_type: o.get_field("authenticationType"),
            created_time: o.get_field("createdTime"),
            enabled: o.get_field("enabled"),
            first_name: o.get_field("firstName"),
            last_name: o.get_field("lastName"),
            send_email_notification: o.get_field("sendEmailNotification"),
            user_name: o.get_field("userName"),
        }
    }
}
