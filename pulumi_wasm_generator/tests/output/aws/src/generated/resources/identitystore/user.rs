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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Details about the user's address. At most 1 address is allowed. Detailed below.
        #[builder(into, default)]
        pub addresses: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserAddresses>,
        >,
        /// The name that is typically displayed when the user is referenced.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Details about the user's email. At most 1 email is allowed. Detailed below.
        #[builder(into, default)]
        pub emails: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserEmails>,
        >,
        /// The globally unique identifier for the identity store that this user is in.
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The user's geographical region or location.
        #[builder(into, default)]
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the user's full name. Detailed below.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserName>,
        >,
        /// An alternate name for the user.
        #[builder(into, default)]
        pub nickname: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the user's phone number. At most 1 phone number is allowed. Detailed below.
        #[builder(into, default)]
        pub phone_numbers: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserPhoneNumbers>,
        >,
        /// The preferred language of the user.
        #[builder(into, default)]
        pub preferred_language: pulumi_wasm_rust::Output<Option<String>>,
        /// An URL that may be associated with the user.
        #[builder(into, default)]
        pub profile_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's time zone.
        #[builder(into, default)]
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's title.
        #[builder(into, default)]
        pub title: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique string used to identify the user. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. The limit is 128 characters.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// The user type.
        #[builder(into, default)]
        pub user_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Details about the user's address. At most 1 address is allowed. Detailed below.
        pub addresses: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserAddresses>,
        >,
        /// The name that is typically displayed when the user is referenced.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Details about the user's email. At most 1 email is allowed. Detailed below.
        pub emails: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserEmails>,
        >,
        /// A list of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_wasm_rust::Output<
            Vec<super::super::types::identitystore::UserExternalId>,
        >,
        /// The globally unique identifier for the identity store that this user is in.
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The user's geographical region or location.
        pub locale: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the user's full name. Detailed below.
        pub name: pulumi_wasm_rust::Output<super::super::types::identitystore::UserName>,
        /// An alternate name for the user.
        pub nickname: pulumi_wasm_rust::Output<Option<String>>,
        /// Details about the user's phone number. At most 1 phone number is allowed. Detailed below.
        pub phone_numbers: pulumi_wasm_rust::Output<
            Option<super::super::types::identitystore::UserPhoneNumbers>,
        >,
        /// The preferred language of the user.
        pub preferred_language: pulumi_wasm_rust::Output<Option<String>>,
        /// An URL that may be associated with the user.
        pub profile_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's time zone.
        pub timezone: pulumi_wasm_rust::Output<Option<String>>,
        /// The user's title.
        pub title: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier for this user in the identity store.
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// A unique string used to identify the user. This value can consist of letters, accented characters, symbols, numbers, and punctuation. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store. The limit is 128 characters.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserArgs) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addresses_binding = args.addresses.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let emails_binding = args.emails.get_inner();
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let locale_binding = args.locale.get_inner();
        let name_binding = args.name.get_inner();
        let nickname_binding = args.nickname.get_inner();
        let phone_numbers_binding = args.phone_numbers.get_inner();
        let preferred_language_binding = args.preferred_language.get_inner();
        let profile_url_binding = args.profile_url.get_inner();
        let timezone_binding = args.timezone.get_inner();
        let title_binding = args.title.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let user_type_binding = args.user_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:identitystore/user:User".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addresses".into(),
                    value: &addresses_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "emails".into(),
                    value: &emails_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "locale".into(),
                    value: &locale_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nickname".into(),
                    value: &nickname_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumbers".into(),
                    value: &phone_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "preferredLanguage".into(),
                    value: &preferred_language_binding,
                },
                register_interface::ObjectField {
                    name: "profileUrl".into(),
                    value: &profile_url_binding,
                },
                register_interface::ObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding,
                },
                register_interface::ObjectField {
                    name: "title".into(),
                    value: &title_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
                register_interface::ObjectField {
                    name: "userType".into(),
                    value: &user_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addresses".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "emails".into(),
                },
                register_interface::ResultField {
                    name: "externalIds".into(),
                },
                register_interface::ResultField {
                    name: "identityStoreId".into(),
                },
                register_interface::ResultField {
                    name: "locale".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nickname".into(),
                },
                register_interface::ResultField {
                    name: "phoneNumbers".into(),
                },
                register_interface::ResultField {
                    name: "preferredLanguage".into(),
                },
                register_interface::ResultField {
                    name: "profileUrl".into(),
                },
                register_interface::ResultField {
                    name: "timezone".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
                register_interface::ResultField {
                    name: "userType".into(),
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
            addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addresses").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            emails: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emails").unwrap(),
            ),
            external_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIds").unwrap(),
            ),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locale").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nickname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nickname").unwrap(),
            ),
            phone_numbers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneNumbers").unwrap(),
            ),
            preferred_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredLanguage").unwrap(),
            ),
            profile_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileUrl").unwrap(),
            ),
            timezone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezone").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
            user_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userType").unwrap(),
            ),
        }
    }
}