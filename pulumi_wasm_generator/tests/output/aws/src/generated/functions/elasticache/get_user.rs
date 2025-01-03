pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// String for what access a user possesses within the associated ElastiCache replication groups or clusters.
        #[builder(into, default)]
        pub access_string: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub authentication_modes: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::elasticache::GetUserAuthenticationMode>,
            >,
        >,
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub no_password_required: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub passwords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Identifier for the user.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// User name of the user.
        #[builder(into, default)]
        pub user_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// String for what access a user possesses within the associated ElastiCache replication groups or clusters.
        pub access_string: pulumi_wasm_rust::Output<Option<String>>,
        pub authentication_modes: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::elasticache::GetUserAuthenticationMode>,
            >,
        >,
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub no_password_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub passwords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Identifier for the user.
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// User name of the user.
        pub user_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetUserArgs) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_string_binding = args.access_string.get_inner();
        let authentication_modes_binding = args.authentication_modes.get_inner();
        let engine_binding = args.engine.get_inner();
        let no_password_required_binding = args.no_password_required.get_inner();
        let passwords_binding = args.passwords.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getUser:getUser".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessString".into(),
                    value: &access_string_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationModes".into(),
                    value: &authentication_modes_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "noPasswordRequired".into(),
                    value: &no_password_required_binding,
                },
                register_interface::ObjectField {
                    name: "passwords".into(),
                    value: &passwords_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessString".into(),
                },
                register_interface::ResultField {
                    name: "authenticationModes".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "noPasswordRequired".into(),
                },
                register_interface::ResultField {
                    name: "passwords".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
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
            access_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessString").unwrap(),
            ),
            authentication_modes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationModes").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            no_password_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noPasswordRequired").unwrap(),
            ),
            passwords: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwords").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
