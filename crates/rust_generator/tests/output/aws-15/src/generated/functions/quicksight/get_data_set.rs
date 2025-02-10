#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataSetArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier for the data set.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub data_set_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub tags_all: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDataSetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        pub column_groups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetColumnGroup>,
        >,
        pub column_level_permission_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetColumnLevelPermissionRule,
            >,
        >,
        pub data_set_id: pulumi_gestalt_rust::Output<String>,
        pub data_set_usage_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetDataSetUsageConfiguration,
            >,
        >,
        pub field_folders: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetFieldFolder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub import_mode: pulumi_gestalt_rust::Output<String>,
        pub logical_table_maps: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetLogicalTableMap>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetPermission>,
        >,
        pub physical_table_maps: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetDataSetPhysicalTableMap>,
        >,
        pub row_level_permission_data_sets: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetRowLevelPermissionDataSet,
            >,
        >,
        pub row_level_permission_tag_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::quicksight::GetDataSetRowLevelPermissionTagConfiguration,
            >,
        >,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDataSetArgs,
    ) -> GetDataSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let data_set_id_binding = args.data_set_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tags_all_binding = args.tags_all.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:quicksight/getDataSet:getDataSet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: aws_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSetId".into(),
                    value: data_set_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagsAll".into(),
                    value: tags_all_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDataSetResult {
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            column_groups: o.get_field("columnGroups"),
            column_level_permission_rules: o.get_field("columnLevelPermissionRules"),
            data_set_id: o.get_field("dataSetId"),
            data_set_usage_configurations: o.get_field("dataSetUsageConfigurations"),
            field_folders: o.get_field("fieldFolders"),
            id: o.get_field("id"),
            import_mode: o.get_field("importMode"),
            logical_table_maps: o.get_field("logicalTableMaps"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            physical_table_maps: o.get_field("physicalTableMaps"),
            row_level_permission_data_sets: o.get_field("rowLevelPermissionDataSets"),
            row_level_permission_tag_configurations: o
                .get_field("rowLevelPermissionTagConfigurations"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
