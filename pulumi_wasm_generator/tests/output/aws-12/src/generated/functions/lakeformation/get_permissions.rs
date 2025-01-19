pub mod get_permissions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionsArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the permissions are to be granted for the Data Catalog. Defaults to `false`.
        #[builder(into, default)]
        pub catalog_resource: pulumi_wasm_rust::Output<Option<bool>>,
        /// Configuration block for a data cells filter resource. Detailed below.
        #[builder(into, default)]
        pub data_cells_filter: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::lakeformation::GetPermissionsDataCellsFilter,
            >,
        >,
        /// Configuration block for a data location resource. Detailed below.
        #[builder(into, default)]
        pub data_location: pulumi_wasm_rust::Output<
            Option<super::super::super::types::lakeformation::GetPermissionsDataLocation>,
        >,
        /// Configuration block for a database resource. Detailed below.
        #[builder(into, default)]
        pub database: pulumi_wasm_rust::Output<
            Option<super::super::super::types::lakeformation::GetPermissionsDatabase>,
        >,
        /// Configuration block for an LF-tag resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag: pulumi_wasm_rust::Output<
            Option<super::super::super::types::lakeformation::GetPermissionsLfTag>,
        >,
        /// Configuration block for an LF-tag policy resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag_policy: pulumi_wasm_rust::Output<
            Option<super::super::super::types::lakeformation::GetPermissionsLfTagPolicy>,
        >,
        /// Principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles.
        ///
        /// One of the following is required:
        #[builder(into)]
        pub principal: pulumi_wasm_rust::Output<String>,
        /// Configuration block for a table resource. Detailed below.
        #[builder(into, default)]
        pub table: pulumi_wasm_rust::Output<
            Option<super::super::super::types::lakeformation::GetPermissionsTable>,
        >,
        /// Configuration block for a table with columns resource. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::lakeformation::GetPermissionsTableWithColumns,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPermissionsResult {
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        pub catalog_resource: pulumi_wasm_rust::Output<Option<bool>>,
        pub data_cells_filter: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDataCellsFilter,
        >,
        pub data_location: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDataLocation,
        >,
        pub database: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDatabase,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub lf_tag: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsLfTag,
        >,
        pub lf_tag_policy: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsLfTagPolicy,
        >,
        /// List of permissions granted to the principal. For details on permissions, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        pub permissions: pulumi_wasm_rust::Output<Vec<String>>,
        /// Subset of `permissions` which the principal can pass.
        pub permissions_with_grant_options: pulumi_wasm_rust::Output<Vec<String>>,
        pub principal: pulumi_wasm_rust::Output<String>,
        pub table: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsTable,
        >,
        pub table_with_columns: pulumi_wasm_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsTableWithColumns,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPermissionsArgs) -> GetPermissionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let catalog_resource_binding = args.catalog_resource.get_inner();
        let data_cells_filter_binding = args.data_cells_filter.get_inner();
        let data_location_binding = args.data_location.get_inner();
        let database_binding = args.database.get_inner();
        let lf_tag_binding = args.lf_tag.get_inner();
        let lf_tag_policy_binding = args.lf_tag_policy.get_inner();
        let principal_binding = args.principal.get_inner();
        let table_binding = args.table.get_inner();
        let table_with_columns_binding = args.table_with_columns.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lakeformation/getPermissions:getPermissions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "catalogResource".into(),
                    value: &catalog_resource_binding,
                },
                register_interface::ObjectField {
                    name: "dataCellsFilter".into(),
                    value: &data_cells_filter_binding,
                },
                register_interface::ObjectField {
                    name: "dataLocation".into(),
                    value: &data_location_binding,
                },
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "lfTag".into(),
                    value: &lf_tag_binding,
                },
                register_interface::ObjectField {
                    name: "lfTagPolicy".into(),
                    value: &lf_tag_policy_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
                register_interface::ObjectField {
                    name: "tableWithColumns".into(),
                    value: &table_with_columns_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "catalogResource".into(),
                },
                register_interface::ResultField {
                    name: "dataCellsFilter".into(),
                },
                register_interface::ResultField {
                    name: "dataLocation".into(),
                },
                register_interface::ResultField {
                    name: "database".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lfTag".into(),
                },
                register_interface::ResultField {
                    name: "lfTagPolicy".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "permissionsWithGrantOptions".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
                register_interface::ResultField {
                    name: "tableWithColumns".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPermissionsResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            catalog_resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogResource").unwrap(),
            ),
            data_cells_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCellsFilter").unwrap(),
            ),
            data_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataLocation").unwrap(),
            ),
            database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("database").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            lf_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lfTag").unwrap(),
            ),
            lf_tag_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lfTagPolicy").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            permissions_with_grant_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionsWithGrantOptions").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
            table_with_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableWithColumns").unwrap(),
            ),
        }
    }
}
