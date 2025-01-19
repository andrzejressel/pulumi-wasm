pub mod flexible_server_active_directory_administratory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratoryArgs {
        #[builder(into)]
        pub identity_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub login: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub object_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub server_id: pulumi_wasm_rust::Output<String>,
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerActiveDirectoryAdministratoryResult {
        pub identity_id: pulumi_wasm_rust::Output<String>,
        pub login: pulumi_wasm_rust::Output<String>,
        pub object_id: pulumi_wasm_rust::Output<String>,
        pub server_id: pulumi_wasm_rust::Output<String>,
        pub tenant_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FlexibleServerActiveDirectoryAdministratoryArgs,
    ) -> FlexibleServerActiveDirectoryAdministratoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_id_binding = args.identity_id.get_inner();
        let login_binding = args.login.get_inner();
        let object_id_binding = args.object_id.get_inner();
        let server_id_binding = args.server_id.get_inner();
        let tenant_id_binding = args.tenant_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mysql/flexibleServerActiveDirectoryAdministratory:FlexibleServerActiveDirectoryAdministratory"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "login".into(),
                    value: &login_binding,
                },
                register_interface::ObjectField {
                    name: "objectId".into(),
                    value: &object_id_binding,
                },
                register_interface::ObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "identityId".into(),
                },
                register_interface::ResultField {
                    name: "login".into(),
                },
                register_interface::ResultField {
                    name: "objectId".into(),
                },
                register_interface::ResultField {
                    name: "serverId".into(),
                },
                register_interface::ResultField {
                    name: "tenantId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlexibleServerActiveDirectoryAdministratoryResult {
            identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityId").unwrap(),
            ),
            login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("login").unwrap(),
            ),
            object_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("objectId").unwrap(),
            ),
            server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverId").unwrap(),
            ),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tenantId").unwrap(),
            ),
        }
    }
}
