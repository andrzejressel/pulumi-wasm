pub mod get_autonomous_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAutonomousDatabaseArgs {
        /// The ID of the AutonomousDatabase.
        #[builder(into)]
        pub autonomous_database_id: pulumi_wasm_rust::Output<String>,
        /// The location of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The project to which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAutonomousDatabaseResult {
        pub admin_password: pulumi_wasm_rust::Output<String>,
        pub autonomous_database_id: pulumi_wasm_rust::Output<String>,
        pub cidr: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub database: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub entitlement_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::oracledatabase::GetAutonomousDatabaseProperty,
            >,
        >,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAutonomousDatabaseArgs) -> GetAutonomousDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autonomous_database_id_binding = args.autonomous_database_id.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:oracledatabase/getAutonomousDatabase:getAutonomousDatabase"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autonomousDatabaseId".into(),
                    value: &autonomous_database_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adminPassword".into(),
                },
                register_interface::ResultField {
                    name: "autonomousDatabaseId".into(),
                },
                register_interface::ResultField {
                    name: "cidr".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "entitlementId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAutonomousDatabaseResult {
            admin_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adminPassword").unwrap(),
            ),
            autonomous_database_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autonomousDatabaseId").unwrap(),
            ),
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            entitlement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entitlementId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
        }
    }
}
