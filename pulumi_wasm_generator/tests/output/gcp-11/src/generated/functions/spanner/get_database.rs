pub mod get_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDatabaseArgs {
        /// The name of the database's spanner instance.
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The name of the spanner database.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDatabaseResult {
        pub database_dialect: pulumi_wasm_rust::Output<String>,
        pub ddls: pulumi_wasm_rust::Output<Vec<String>>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub enable_drop_protection: pulumi_wasm_rust::Output<bool>,
        pub encryption_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::spanner::GetDatabaseEncryptionConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub version_retention_period: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDatabaseArgs) -> GetDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:spanner/getDatabase:getDatabase".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "databaseDialect".into(),
                },
                register_interface::ResultField {
                    name: "ddls".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "enableDropProtection".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfigs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "versionRetentionPeriod".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDatabaseResult {
            database_dialect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseDialect").unwrap(),
            ),
            ddls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ddls").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            enable_drop_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDropProtection").unwrap(),
            ),
            encryption_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfigs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            version_retention_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionRetentionPeriod").unwrap(),
            ),
        }
    }
}
