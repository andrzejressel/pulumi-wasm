pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific User by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the User.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Returns information on a specific User by User id
        #[builder(into, default)]
        pub user_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// The Amazon Resource Name (ARN) of the User.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the user account in the directory used for identity management.
        pub directory_user_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the hierarchy group for the user.
        pub hierarchy_group_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A block that contains information about the identity of the user. Documented below.
        pub identity_infos: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetUserIdentityInfo>,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A block that contains information about the phone settings for the user. Documented below.
        pub phone_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetUserPhoneConfig>,
        >,
        /// The identifier of the routing profile for the user.
        pub routing_profile_id: pulumi_wasm_rust::Output<String>,
        /// A list of identifiers for the security profiles for the user.
        pub security_profile_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the User.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_id: pulumi_wasm_rust::Output<String>,
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
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            directory_user_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryUserId"),
            ),
            hierarchy_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hierarchyGroupId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identity_infos: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityInfos"),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            phone_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("phoneConfigs"),
            ),
            routing_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routingProfileId"),
            ),
            security_profile_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityProfileIds"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
        }
    }
}
