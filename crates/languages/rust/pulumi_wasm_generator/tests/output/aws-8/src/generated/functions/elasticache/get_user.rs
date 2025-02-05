pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// String for what access a user possesses within the associated ElastiCache replication groups or clusters.
        #[builder(into, default)]
        pub access_string: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub authentication_modes: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::super::types::elasticache::GetUserAuthenticationMode>,
            >,
        >,
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub no_password_required: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub passwords: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Identifier for the user.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// User name of the user.
        #[builder(into, default)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_string_binding = args.access_string.get_output(context).get_inner();
        let authentication_modes_binding = args
            .authentication_modes
            .get_output(context)
            .get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let no_password_required_binding = args
            .no_password_required
            .get_output(context)
            .get_inner();
        let passwords_binding = args.passwords.get_output(context).get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticache/getUser:getUser".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            access_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessString"),
            ),
            authentication_modes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authenticationModes"),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(o.extract_field("engine")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            no_password_required: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("noPasswordRequired"),
            ),
            passwords: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("passwords"),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
