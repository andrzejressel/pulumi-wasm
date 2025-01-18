pub mod get_data_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSetArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier for the data set.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub tags_all: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDataSetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        pub column_groups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetColumnGroup>,
        >,
        pub column_level_permission_rules: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetColumnLevelPermissionRule,
            >,
        >,
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        pub data_set_usage_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetDataSetUsageConfiguration,
            >,
        >,
        pub field_folders: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetFieldFolder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub import_mode: pulumi_wasm_rust::Output<String>,
        pub logical_table_maps: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetLogicalTableMap>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetPermission>,
        >,
        pub physical_table_maps: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetPhysicalTableMap>,
        >,
        pub row_level_permission_data_sets: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetRowLevelPermissionDataSet,
            >,
        >,
        pub row_level_permission_tag_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetRowLevelPermissionTagConfiguration,
            >,
        >,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDataSetArgs) -> GetDataSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let data_set_id_binding = args.data_set_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let tags_all_binding = args.tags_all.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:quicksight/getDataSet:getDataSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tagsAll".into(),
                    value: &tags_all_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "columnGroups".into(),
                },
                register_interface::ResultField {
                    name: "columnLevelPermissionRules".into(),
                },
                register_interface::ResultField {
                    name: "dataSetId".into(),
                },
                register_interface::ResultField {
                    name: "dataSetUsageConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "fieldFolders".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "importMode".into(),
                },
                register_interface::ResultField {
                    name: "logicalTableMaps".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "physicalTableMaps".into(),
                },
                register_interface::ResultField {
                    name: "rowLevelPermissionDataSets".into(),
                },
                register_interface::ResultField {
                    name: "rowLevelPermissionTagConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            column_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("columnGroups").unwrap(),
            ),
            column_level_permission_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("columnLevelPermissionRules").unwrap(),
            ),
            data_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetId").unwrap(),
            ),
            data_set_usage_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetUsageConfigurations").unwrap(),
            ),
            field_folders: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fieldFolders").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            import_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importMode").unwrap(),
            ),
            logical_table_maps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicalTableMaps").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            physical_table_maps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("physicalTableMaps").unwrap(),
            ),
            row_level_permission_data_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowLevelPermissionDataSets").unwrap(),
            ),
            row_level_permission_tag_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowLevelPermissionTagConfigurations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
