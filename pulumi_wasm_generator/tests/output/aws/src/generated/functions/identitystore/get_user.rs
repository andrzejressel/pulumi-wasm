pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// A unique identifier for a user or group that is not the primary identifier. Conflicts with `user_id` and `filter`. Detailed below.
        #[builder(into, default)]
        pub alternate_identifier: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetUserAlternateIdentifier>,
        >,
        /// Configuration block for filtering by a unique attribute of the user. Detailed below.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetUserFilter>,
        >,
        /// Identity Store ID associated with the Single Sign-On Instance.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The identifier for a user in the Identity Store.
        ///
        /// > Exactly one of the above arguments must be provided. Passing both `filter` and `user_id` is allowed for backwards compatibility.
        #[builder(into, default)]
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// List of details about the user's address.
        pub addresses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserAddress>,
        >,
        pub alternate_identifier: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetUserAlternateIdentifier>,
        >,
        /// The name that is typically displayed when the user is referenced.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// List of details about the user's email.
        pub emails: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserEmail>,
        >,
        /// List of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserExternalId>,
        >,
        pub filter: pulumi_wasm_rust::Output<
            Option<super::super::super::types::identitystore::GetUserFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
        /// The user's geographical region or location.
        pub locale: pulumi_wasm_rust::Output<String>,
        /// Details about the user's full name.
        pub names: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserName>,
        >,
        /// An alternate name for the user.
        pub nickname: pulumi_wasm_rust::Output<String>,
        /// List of details about the user's phone number.
        pub phone_numbers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserPhoneNumber>,
        >,
        /// The preferred language of the user.
        pub preferred_language: pulumi_wasm_rust::Output<String>,
        /// An URL that may be associated with the user.
        pub profile_url: pulumi_wasm_rust::Output<String>,
        /// The user's time zone.
        pub timezone: pulumi_wasm_rust::Output<String>,
        /// The user's title.
        pub title: pulumi_wasm_rust::Output<String>,
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// User's user name value.
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetUserArgs) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alternate_identifier_binding = args.alternate_identifier.get_inner();
        let filter_binding = args.filter.get_inner();
        let identity_store_id_binding = args.identity_store_id.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getUser:getUser".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alternateIdentifier".into(),
                    value: &alternate_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addresses".into(),
                },
                register_interface::ResultField {
                    name: "alternateIdentifier".into(),
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
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityStoreId".into(),
                },
                register_interface::ResultField {
                    name: "locale".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserResult {
            addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addresses").unwrap(),
            ),
            alternate_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alternateIdentifier").unwrap(),
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
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityStoreId").unwrap(),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locale").unwrap(),
            ),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
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
