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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "directoryUserId".into(),
                },
                register_interface::ResultField {
                    name: "hierarchyGroupId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityInfos".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "phoneConfigs".into(),
                },
                register_interface::ResultField {
                    name: "routingProfileId".into(),
                },
                register_interface::ResultField {
                    name: "securityProfileIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            directory_user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryUserId").unwrap(),
            ),
            hierarchy_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hierarchyGroupId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityInfos").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            phone_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneConfigs").unwrap(),
            ),
            routing_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingProfileId").unwrap(),
            ),
            security_profile_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityProfileIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
        }
    }
}
