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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserProfileArgs,
    ) -> UserProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_identifier_binding_1 = args.domain_identifier.get_output(context);
        let domain_identifier_binding = domain_identifier_binding_1.get_inner();
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let user_identifier_binding_1 = args.user_identifier.get_output(context);
        let user_identifier_binding = user_identifier_binding_1.get_inner();
        let user_type_binding_1 = args.user_type.get_output(context);
        let user_type_binding = user_type_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "userIdentifier".into(),
                    value: &user_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "userType".into(),
                    value: &user_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserProfileResult {
            details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("details"),
            ),
            domain_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIdentifier"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            user_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userIdentifier"),
            ),
            user_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userType"),
            ),
        }
    }
}
