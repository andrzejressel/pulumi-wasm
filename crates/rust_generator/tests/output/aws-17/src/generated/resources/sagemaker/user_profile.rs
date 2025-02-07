/// Provides a SageMaker User Profile resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user_profile::create(
///         "example",
///         UserProfileArgs::builder()
///             .domain_id("${test.id}")
///             .user_profile_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker User Profiles using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/userProfile:UserProfile test_user_profile arn:aws:sagemaker:us-west-2:123456789012:user-profile/domain-id/profile-name
/// ```
pub mod user_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// The ID of the associated Domain.
        #[builder(into)]
        pub domain_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A specifier for the type of value specified in `single_sign_on_user_value`. Currently, the only supported value is `UserName`. If the Domain's AuthMode is SSO, this field is required. If the Domain's AuthMode is not SSO, this field cannot be specified.
        #[builder(into, default)]
        pub single_sign_on_user_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The username of the associated AWS Single Sign-On User for this User Profile. If the Domain's AuthMode is SSO, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not SSO, this field cannot be specified.
        #[builder(into, default)]
        pub single_sign_on_user_value: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name for the User Profile.
        #[builder(into)]
        pub user_profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user settings. See User Settings below.
        #[builder(into, default)]
        pub user_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sagemaker::UserProfileUserSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// The user profile Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the user's profile in the Amazon Elastic File System (EFS) volume.
        pub home_efs_file_system_uid: pulumi_gestalt_rust::Output<String>,
        /// A specifier for the type of value specified in `single_sign_on_user_value`. Currently, the only supported value is `UserName`. If the Domain's AuthMode is SSO, this field is required. If the Domain's AuthMode is not SSO, this field cannot be specified.
        pub single_sign_on_user_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The username of the associated AWS Single Sign-On User for this User Profile. If the Domain's AuthMode is SSO, this field is required, and must match a valid username of a user in your directory. If the Domain's AuthMode is not SSO, this field cannot be specified.
        pub single_sign_on_user_value: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name for the User Profile.
        pub user_profile_name: pulumi_gestalt_rust::Output<String>,
        /// The user settings. See User Settings below.
        pub user_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::sagemaker::UserProfileUserSettings>,
        >,
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
        let domain_id_binding = args.domain_id.get_output(context).get_inner();
        let single_sign_on_user_identifier_binding = args
            .single_sign_on_user_identifier
            .get_output(context)
            .get_inner();
        let single_sign_on_user_value_binding = args
            .single_sign_on_user_value
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_profile_name_binding = args
            .user_profile_name
            .get_output(context)
            .get_inner();
        let user_settings_binding = args.user_settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "singleSignOnUserIdentifier".into(),
                    value: &single_sign_on_user_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "singleSignOnUserValue".into(),
                    value: &single_sign_on_user_value_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userProfileName".into(),
                    value: &user_profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "userSettings".into(),
                    value: &user_settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserProfileResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            domain_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainId"),
            ),
            home_efs_file_system_uid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("homeEfsFileSystemUid"),
            ),
            single_sign_on_user_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("singleSignOnUserIdentifier"),
            ),
            single_sign_on_user_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("singleSignOnUserValue"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            user_profile_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userProfileName"),
            ),
            user_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userSettings"),
            ),
        }
    }
}
