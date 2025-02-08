/// Provides an OpsWorks User Profile resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myProfile = user_profile::create(
///         "myProfile",
///         UserProfileArgs::builder()
///             .ssh_username("my_user")
///             .user_arn("${user.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// Whether users can specify their own SSH public key through the My Settings page
        #[builder(into, default)]
        pub allow_self_management: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The users public key
        #[builder(into, default)]
        pub ssh_public_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        #[builder(into)]
        pub ssh_username: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user's IAM ARN
        #[builder(into)]
        pub user_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Whether users can specify their own SSH public key through the My Settings page
        pub allow_self_management: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The users public key
        pub ssh_public_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ssh username, with witch this user wants to log in
        pub ssh_username: pulumi_gestalt_rust::Output<String>,
        /// The user's IAM ARN
        pub user_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserProfileArgs,
    ) -> UserProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allow_self_management_binding = args
            .allow_self_management
            .get_output(context)
            .get_inner();
        let ssh_public_key_binding = args.ssh_public_key.get_output(context).get_inner();
        let ssh_username_binding = args.ssh_username.get_output(context).get_inner();
        let user_arn_binding = args.user_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opsworks/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowSelfManagement".into(),
                    value: &allow_self_management_binding,
                },
                register_interface::ObjectField {
                    name: "sshPublicKey".into(),
                    value: &ssh_public_key_binding,
                },
                register_interface::ObjectField {
                    name: "sshUsername".into(),
                    value: &ssh_username_binding,
                },
                register_interface::ObjectField {
                    name: "userArn".into(),
                    value: &user_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserProfileResult {
            allow_self_management: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowSelfManagement"),
            ),
            ssh_public_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sshPublicKey"),
            ),
            ssh_username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sshUsername"),
            ),
            user_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userArn"),
            ),
        }
    }
}
