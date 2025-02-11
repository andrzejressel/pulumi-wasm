#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// A unique identifier for a user or group that is not the primary identifier. Conflicts with `user_id` and `filter`. Detailed below.
        #[builder(into, default)]
        pub alternate_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::identitystore::GetUserAlternateIdentifier>,
        >,
        /// Configuration block for filtering by a unique attribute of the user. Detailed below.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::identitystore::GetUserFilter>,
        >,
        /// Identity Store ID associated with the Single Sign-On Instance.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub identity_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identifier for a user in the Identity Store.
        ///
        /// > Exactly one of the above arguments must be provided. Passing both `filter` and `user_id` is allowed for backwards compatibility.
        #[builder(into, default)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// List of details about the user's address.
        pub addresses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserAddress>,
        >,
        pub alternate_identifier: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::identitystore::GetUserAlternateIdentifier>,
        >,
        /// The name that is typically displayed when the user is referenced.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// List of details about the user's email.
        pub emails: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserEmail>,
        >,
        /// List of identifiers issued to this resource by an external identity provider.
        pub external_ids: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserExternalId>,
        >,
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::identitystore::GetUserFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identity_store_id: pulumi_gestalt_rust::Output<String>,
        /// The user's geographical region or location.
        pub locale: pulumi_gestalt_rust::Output<String>,
        /// Details about the user's full name.
        pub names: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserName>,
        >,
        /// An alternate name for the user.
        pub nickname: pulumi_gestalt_rust::Output<String>,
        /// List of details about the user's phone number.
        pub phone_numbers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::identitystore::GetUserPhoneNumber>,
        >,
        /// The preferred language of the user.
        pub preferred_language: pulumi_gestalt_rust::Output<String>,
        /// An URL that may be associated with the user.
        pub profile_url: pulumi_gestalt_rust::Output<String>,
        /// The user's time zone.
        pub timezone: pulumi_gestalt_rust::Output<String>,
        /// The user's title.
        pub title: pulumi_gestalt_rust::Output<String>,
        pub user_id: pulumi_gestalt_rust::Output<String>,
        /// User's user name value.
        pub user_name: pulumi_gestalt_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserArgs,
    ) -> GetUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alternate_identifier_binding = args.alternate_identifier.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let identity_store_id_binding = args.identity_store_id.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:identitystore/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alternateIdentifier".into(),
                    value: &alternate_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: &user_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserResult {
            addresses: o.get_field("addresses"),
            alternate_identifier: o.get_field("alternateIdentifier"),
            display_name: o.get_field("displayName"),
            emails: o.get_field("emails"),
            external_ids: o.get_field("externalIds"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            identity_store_id: o.get_field("identityStoreId"),
            locale: o.get_field("locale"),
            names: o.get_field("names"),
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
