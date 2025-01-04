pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// Map of tags assigned to the user.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the user.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        /// Access permissions string used for this user.
        pub access_string: pulumi_wasm_rust::Output<String>,
        /// ARN of the user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Denotes the user's authentication properties.
        pub authentication_modes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::memorydb::GetUserAuthenticationMode>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Minimum engine version supported for the user.
        pub minimum_engine_version: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the user.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetUserArgs) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:memorydb/getUser:getUser".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authenticationModes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "minimumEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
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
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_modes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationModes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            minimum_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumEngineVersion").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
