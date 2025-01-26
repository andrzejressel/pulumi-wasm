pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// A unique identifier for a user or group that is not the primary identifier. Conflicts with `user_id` and `filter`. Detailed below.
        #[builder(into, default)]
        pub alternate_identifier: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::super::types::identitystore::GetUserAlternateIdentifier>,
        >,
        /// Configuration block for filtering by a unique attribute of the user. Detailed below.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::super::types::identitystore::GetUserFilter>,
        >,
        /// Identity Store ID associated with the Single Sign-On Instance.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifier for a user in the Identity Store.
        ///
        /// > Exactly one of the above arguments must be provided. Passing both `filter` and `user_id` is allowed for backwards compatibility.
        #[builder(into, default)]
        pub user_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alternate_identifier_binding = args
            .alternate_identifier
            .get_output(context)
            .get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let identity_store_id_binding = args
            .identity_store_id
            .get_output(context)
            .get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getUser:getUser".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addresses"),
            ),
            alternate_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("alternateIdentifier"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            emails: pulumi_wasm_rust::__private::into_domain(o.extract_field("emails")),
            external_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("externalIds"),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityStoreId"),
            ),
            locale: pulumi_wasm_rust::__private::into_domain(o.extract_field("locale")),
            names: pulumi_wasm_rust::__private::into_domain(o.extract_field("names")),
            nickname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nickname"),
            ),
            phone_numbers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("phoneNumbers"),
            ),
            preferred_language: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preferredLanguage"),
            ),
            profile_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("profileUrl"),
            ),
            timezone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timezone"),
            ),
            title: pulumi_wasm_rust::__private::into_domain(o.extract_field("title")),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
            user_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userType"),
            ),
        }
    }
}
