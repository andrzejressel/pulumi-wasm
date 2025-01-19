pub mod get_data_lake_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataLakeSettingsArgs {
        /// Identifier for the Data Catalog. By default, the account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDataLakeSettingsResult {
        /// List of ARNs of AWS Lake Formation principals (IAM users or roles).
        pub admins: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether to allow Amazon EMR clusters to access data managed by Lake Formation.
        pub allow_external_data_filtering: pulumi_wasm_rust::Output<bool>,
        /// Whether to allow a third-party query engine to get data access credentials without session tags when a caller has full data access permissions.
        pub allow_full_table_external_data_access: pulumi_wasm_rust::Output<bool>,
        /// Lake Formation relies on a privileged process secured by Amazon EMR or the third party integrator to tag the user's role while assuming it.
        pub authorized_session_tag_value_lists: pulumi_wasm_rust::Output<Vec<String>>,
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Up to three configuration blocks of principal permissions for default create database permissions. Detailed below.
        pub create_database_default_permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::lakeformation::GetDataLakeSettingsCreateDatabaseDefaultPermission,
            >,
        >,
        /// Up to three configuration blocks of principal permissions for default create table permissions. Detailed below.
        pub create_table_default_permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::lakeformation::GetDataLakeSettingsCreateTableDefaultPermission,
            >,
        >,
        /// A list of the account IDs of Amazon Web Services accounts with Amazon EMR clusters that are to perform data filtering.
        pub external_data_filtering_allow_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of additional configuration. `CROSS_ACCOUNT_VERSION` will be set to values `"1"`, `"2"`, `"3"`, or `"4"`. `SET_CONTEXT` will also be returned with a value of `TRUE`. In a fresh account, prior to configuring, `CROSS_ACCOUNT_VERSION` is `"1"`.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of ARNs of AWS Lake Formation principals (IAM users or roles) with only view access to the resources.
        pub read_only_admins: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs).
        pub trusted_resource_owners: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDataLakeSettingsArgs) -> GetDataLakeSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lakeformation/getDataLakeSettings:getDataLakeSettings".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "admins".into(),
                },
                register_interface::ResultField {
                    name: "allowExternalDataFiltering".into(),
                },
                register_interface::ResultField {
                    name: "allowFullTableExternalDataAccess".into(),
                },
                register_interface::ResultField {
                    name: "authorizedSessionTagValueLists".into(),
                },
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "createDatabaseDefaultPermissions".into(),
                },
                register_interface::ResultField {
                    name: "createTableDefaultPermissions".into(),
                },
                register_interface::ResultField {
                    name: "externalDataFilteringAllowLists".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "readOnlyAdmins".into(),
                },
                register_interface::ResultField {
                    name: "trustedResourceOwners".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataLakeSettingsResult {
            admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("admins").unwrap(),
            ),
            allow_external_data_filtering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowExternalDataFiltering").unwrap(),
            ),
            allow_full_table_external_data_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowFullTableExternalDataAccess").unwrap(),
            ),
            authorized_session_tag_value_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedSessionTagValueLists").unwrap(),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            create_database_default_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createDatabaseDefaultPermissions").unwrap(),
            ),
            create_table_default_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTableDefaultPermissions").unwrap(),
            ),
            external_data_filtering_allow_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalDataFilteringAllowLists").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            read_only_admins: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readOnlyAdmins").unwrap(),
            ),
            trusted_resource_owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustedResourceOwners").unwrap(),
            ),
        }
    }
}
