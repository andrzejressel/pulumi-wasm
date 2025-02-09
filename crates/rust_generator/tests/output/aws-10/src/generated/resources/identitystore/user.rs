/// This resource manages a User resource within an Identity Store.
///
/// > **Note:** If you use an external identity provider or Active Directory as your identity source,
/// use this resource with caution. IAM Identity Center does not support outbound synchronization,
/// so your identity source does not automatically update with the changes that you make to
/// users using this resource.
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
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .display_name("John Doe")
///             .emails(UserEmails::builder().value("john@example.com").build_struct())
///             .identity_store_id("${exampleAwsSsoadminInstances.identityStoreIds[0]}")
///             .name(UserName::builder().familyName("Doe").givenName("John").build_struct())
///             .user_name("johndoe")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Identity Store User using the combination `identity_store_id/user_id`. For example:
///
/// ```sh
/// $ pulumi import aws:identitystore/user:User example d-9c6705e95c/065212b4-9061-703b-5876-13a517ae2a7c
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Details about the user's address. At most 1 address is allowed. Detailed below.
        #[builder(into, default)]
        pub addresses: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identitystore::UserAddresses>,
        >,
        /// The name that is typically displayed when the user is referenced.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Details about the user's email. At most 1 email is allowed. Detailed below.
        #[builder(into, default)]
        pub emails: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identitystore::UserEmails>,
        >,
        /// The globally unique identifier for the identity store that this user is in.
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user's geographical region or location.
        #[builder(into, default)]
        pub locale: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the user's full name. Detailed below.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identitystore::UserName>,
        >,
        /// An alternate name for the user.
        #[builder(into, default)]
        pub nickname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details about the user's phone number. At most 1 phone number is allowed. Detailed below.
        #[builder(into, default)]
        pub phone_numbers: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::identitystore::UserPhoneNumbers>,
        >,
        /// The preferred language of the user.
        #[builder(into, default)]
        pub preferred_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An URL that may be associated with the user.
        #[builder(into, default)]
        pub profile_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user's time zone.
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user's title.
        #[builder(into, default)]
        pub title: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A unique string used to identify the user. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. The limit is 128 characters.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The user type.
        #[builder(into, default)]
        pub user_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Details about the user's address. At most 1 address is allowed. Detailed below.
        pub addresses: pulumi_gestalt_rust::Output<
            Option<super::super::types::identitystore::UserAddresses>,
        >,
        /// The name that is typically displayed when the user is referenced.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Details about the user's email. At most 1 email is allowed. Detailed below.
        pub emails: pulumi_gestalt_rust::Output<
            Option<super::super::types::identitystore::UserEmails>,
        >,
        /// A list of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_gestalt_rust::Output<
            Vec<super::super::types::identitystore::UserExternalId>,
        >,
        /// The globally unique identifier for the identity store that this user is in.
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
        /// The user's geographical region or location.
        pub locale: pulumi_gestalt_rust::Output<Option<String>>,
        /// Details about the user's full name. Detailed below.
        pub name: pulumi_gestalt_rust::Output<
            super::super::types::identitystore::UserName,
        >,
        /// An alternate name for the user.
        pub nickname: pulumi_gestalt_rust::Output<Option<String>>,
        /// Details about the user's phone number. At most 1 phone number is allowed. Detailed below.
        pub phone_numbers: pulumi_gestalt_rust::Output<
            Option<super::super::types::identitystore::UserPhoneNumbers>,
        >,
        /// The preferred language of the user.
        pub preferred_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// An URL that may be associated with the user.
        pub profile_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user's time zone.
        pub timezone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The user's title.
        pub title: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier for this user in the identity store.
        pub user_id: pulumi_gestalt_rust::Output<String>,
        /// A unique string used to identify the user. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. The limit is 128 characters.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_gestalt_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_gestalt_rust::Output<Option<String>>,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addresses_binding = args.addresses.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let emails_binding = args.emails.get_output(context);
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let locale_binding = args.locale.get_output(context);
        let name_binding = args.name.get_output(context);
        let nickname_binding = args.nickname.get_output(context);
        let phone_numbers_binding = args.phone_numbers.get_output(context);
        let preferred_language_binding = args.preferred_language.get_output(context);
        let profile_url_binding = args.profile_url.get_output(context);
        let timezone_binding = args.timezone.get_output(context);
        let title_binding = args.title.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let user_type_binding = args.user_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:identitystore/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addresses".into(),
                    value: addresses_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emails".into(),
                    value: emails_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: identity_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locale".into(),
                    value: locale_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nickname".into(),
                    value: nickname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumbers".into(),
                    value: phone_numbers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredLanguage".into(),
                    value: preferred_language_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileUrl".into(),
                    value: profile_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timezone".into(),
                    value: timezone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "title".into(),
                    value: title_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userType".into(),
                    value: user_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            addresses: o.get_field("addresses"),
            display_name: o.get_field("displayName"),
            emails: o.get_field("emails"),
            external_ids: o.get_field("externalIds"),
            identity_store_id: o.get_field("identityStoreId"),
            locale: o.get_field("locale"),
            name: o.get_field("name"),
            nickname: o.get_field("nickname"),
            phone_numbers: o.get_field("phoneNumbers"),
            preferred_language: o.get_field("preferredLanguage"),
            profile_url: o.get_field("profileUrl"),
            timezone: o.get_field("timezone"),
            title: o.get_field("title"),
            user_id: o.get_field("userId"),
            user_name: o.get_field("userName"),
            user_type: o.get_field("userType"),
        }
    }
}
