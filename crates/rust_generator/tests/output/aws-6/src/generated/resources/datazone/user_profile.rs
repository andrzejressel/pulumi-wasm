/// Resource for managing an AWS DataZone User Profile.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_profile::create(
///         "example",
///         UserProfileArgs::builder()
///             .domain_identifier("${exampleAwsDatazoneDomain.id}")
///             .user_identifier("${exampleAwsIamUser.arn}")
///             .user_type("IAM_USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone User Profile using the `user_identifier,domain_identifier,type`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/userProfile:UserProfile example arn:aws:iam::123456789012:user/example,dzd_54nakfrg9k6suo,IAM
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// The domain identifier.
        #[builder(into)]
        pub domain_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user profile status.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user type.
        #[builder(into, default)]
        pub user_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Details about the user profile.
        pub details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::datazone::UserProfileDetail>,
        >,
        /// The domain identifier.
        pub domain_identifier: pulumi_gestalt_rust::Output<String>,
        /// The user profile status.
        pub status: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user profile type.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        pub user_identifier: pulumi_gestalt_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserProfileArgs,
    ) -> UserProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_identifier_binding = args.domain_identifier.get_output(context);
        let status_binding = args.status.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let user_identifier_binding = args.user_identifier.get_output(context);
        let user_type_binding = args.user_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:datazone/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainIdentifier".into(),
                    value: domain_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userIdentifier".into(),
                    value: user_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userType".into(),
                    value: user_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserProfileResult {
            details: o.get_field("details"),
            domain_identifier: o.get_field("domainIdentifier"),
            status: o.get_field("status"),
            timeouts: o.get_field("timeouts"),
            type_: o.get_field("type"),
            user_identifier: o.get_field("userIdentifier"),
            user_type: o.get_field("userType"),
        }
    }
}
