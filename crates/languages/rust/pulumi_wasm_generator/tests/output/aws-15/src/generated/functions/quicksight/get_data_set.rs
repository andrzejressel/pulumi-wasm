pub mod get_data_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSetArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier for the data set.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub tags_all: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDataSetArgs,
    ) -> GetDataSetResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let data_set_id_binding = args.data_set_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tags_all_binding = args.tags_all.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDataSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            column_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("columnGroups"),
            ),
            column_level_permission_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("columnLevelPermissionRules"),
            ),
            data_set_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSetId"),
            ),
            data_set_usage_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataSetUsageConfigurations"),
            ),
            field_folders: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fieldFolders"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            import_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("importMode"),
            ),
            logical_table_maps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logicalTableMaps"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            physical_table_maps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("physicalTableMaps"),
            ),
            row_level_permission_data_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rowLevelPermissionDataSets"),
            ),
            row_level_permission_tag_configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rowLevelPermissionTagConfigurations"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
