pub mod get_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTableArgs {
        /// Name of the Timestream database.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// Name of the Timestream table.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetTableResult {
        /// ARN that uniquely identifies the table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Time that table was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Name of database.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Last time table was updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// Object containing the following attributes to desribe magnetic store writes.
        pub magnetic_store_write_properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::timestreamwrite::GetTableMagneticStoreWriteProperty,
            >,
        >,
        /// Name of the table.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Object containing the following attributes to describe the retention duration for the memory and magnetic stores.
        pub retention_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::timestreamwrite::GetTableRetentionProperty>,
        >,
        /// Object containing the following attributes to describe the schema of the table.
        pub schemas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::timestreamwrite::GetTableSchema>,
        >,
        /// Current state of table.
        pub table_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetTableArgs) -> GetTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let database_name_binding = args.database_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:timestreamwrite/getTable:getTable".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "magneticStoreWriteProperties".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "retentionProperties".into(),
                },
                register_interface::ResultField {
                    name: "schemas".into(),
                },
                register_interface::ResultField {
                    name: "tableStatus".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetTableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            magnetic_store_write_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("magneticStoreWriteProperties").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            retention_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionProperties").unwrap(),
            ),
            schemas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemas").unwrap(),
            ),
            table_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableStatus").unwrap(),
            ),
        }
    }
}
