/// Resource for managing a QuickSight Data Set.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_set::create(
///         "example",
///         DataSetArgs::builder()
///             .data_set_id("example-id")
///             .import_mode("SPICE")
///             .name("example-name")
///             .physical_table_maps(
///                 vec![
///                     DataSetPhysicalTableMap::builder().physicalTableMapId("example-id")
///                     .s3Source(DataSetPhysicalTableMapS3Source::builder()
///                     .dataSourceArn("${exampleAwsQuicksightDataSource.arn}")
///                     .inputColumns(vec![DataSetPhysicalTableMapS3SourceInputColumn::builder()
///                     .name("Column1"). type ("STRING").build_struct(),])
///                     .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettings::builder()
///                     .format("JSON").build_struct()).build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Column Level Permission Rules
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_set::create(
///         "example",
///         DataSetArgs::builder()
///             .column_level_permission_rules(
///                 vec![
///                     DataSetColumnLevelPermissionRule::builder()
///                     .columnNames(vec!["Column1",])
///                     .principals(vec!["${exampleAwsQuicksightUser.arn}",]).build_struct(),
///                 ],
///             )
///             .data_set_id("example-id")
///             .import_mode("SPICE")
///             .name("example-name")
///             .physical_table_maps(
///                 vec![
///                     DataSetPhysicalTableMap::builder().physicalTableMapId("example-id")
///                     .s3Source(DataSetPhysicalTableMapS3Source::builder()
///                     .dataSourceArn("${exampleAwsQuicksightDataSource.arn}")
///                     .inputColumns(vec![DataSetPhysicalTableMapS3SourceInputColumn::builder()
///                     .name("Column1"). type ("STRING").build_struct(),])
///                     .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettings::builder()
///                     .format("JSON").build_struct()).build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Field Folders
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_set::create(
///         "example",
///         DataSetArgs::builder()
///             .data_set_id("example-id")
///             .field_folders(
///                 vec![
///                     DataSetFieldFolder::builder().columns(vec!["Column1",])
///                     .description("example description").fieldFoldersId("example-id")
///                     .build_struct(),
///                 ],
///             )
///             .import_mode("SPICE")
///             .name("example-name")
///             .physical_table_maps(
///                 vec![
///                     DataSetPhysicalTableMap::builder().physicalTableMapId("example-id")
///                     .s3Source(DataSetPhysicalTableMapS3Source::builder()
///                     .dataSourceArn("${exampleAwsQuicksightDataSource.arn}")
///                     .inputColumns(vec![DataSetPhysicalTableMapS3SourceInputColumn::builder()
///                     .name("Column1"). type ("STRING").build_struct(),])
///                     .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettings::builder()
///                     .format("JSON").build_struct()).build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Permissions
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_set::create(
///         "example",
///         DataSetArgs::builder()
///             .data_set_id("example-id")
///             .import_mode("SPICE")
///             .name("example-name")
///             .permissions(
///                 vec![
///                     DataSetPermission::builder()
///                     .actions(vec!["quicksight:DescribeDataSet",
///                     "quicksight:DescribeDataSetPermissions", "quicksight:PassDataSet",
///                     "quicksight:DescribeIngestion", "quicksight:ListIngestions",])
///                     .principal("${exampleAwsQuicksightUser.arn}").build_struct(),
///                 ],
///             )
///             .physical_table_maps(
///                 vec![
///                     DataSetPhysicalTableMap::builder().physicalTableMapId("example-id")
///                     .s3Source(DataSetPhysicalTableMapS3Source::builder()
///                     .dataSourceArn("${exampleAwsQuicksightDataSource.arn}")
///                     .inputColumns(vec![DataSetPhysicalTableMapS3SourceInputColumn::builder()
///                     .name("Column1"). type ("STRING").build_struct(),])
///                     .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettings::builder()
///                     .format("JSON").build_struct()).build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With Row Level Permission Tag Configuration
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_set::create(
///         "example",
///         DataSetArgs::builder()
///             .data_set_id("example-id")
///             .import_mode("SPICE")
///             .name("example-name")
///             .physical_table_maps(
///                 vec![
///                     DataSetPhysicalTableMap::builder().physicalTableMapId("example-id")
///                     .s3Source(DataSetPhysicalTableMapS3Source::builder()
///                     .dataSourceArn("${exampleAwsQuicksightDataSource.arn}")
///                     .inputColumns(vec![DataSetPhysicalTableMapS3SourceInputColumn::builder()
///                     .name("Column1"). type ("STRING").build_struct(),])
///                     .uploadSettings(DataSetPhysicalTableMapS3SourceUploadSettings::builder()
///                     .format("JSON").build_struct()).build_struct()).build_struct(),
///                 ],
///             )
///             .row_level_permission_tag_configuration(
///                 DataSetRowLevelPermissionTagConfiguration::builder()
///                     .status("ENABLED")
///                     .tagRules(
///                         vec![
///                             DataSetRowLevelPermissionTagConfigurationTagRule::builder()
///                             .columnName("Column1").matchAllValue("*").tagKey("tagkey")
///                             .tagMultiValueDelimiter(",").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Data Set using the AWS account ID and data set ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/dataSet:DataSet example 123456789012,example-id
/// ```
pub mod data_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataSetArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported. See column_groups.
        #[builder(into, default)]
        pub column_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetColumnGroup>>,
        >,
        /// A set of 1 or more definitions of a [ColumnLevelPermissionRule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ColumnLevelPermissionRule.html). See column_level_permission_rules.
        #[builder(into, default)]
        pub column_level_permission_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::quicksight::DataSetColumnLevelPermissionRule>,
            >,
        >,
        /// Identifier for the data set.
        #[builder(into)]
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// The usage configuration to apply to child datasets that reference this dataset as a source. See data_set_usage_configuration.
        #[builder(into, default)]
        pub data_set_usage_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSetDataSetUsageConfiguration>,
        >,
        /// The folder that contains fields and nested subfolders for your dataset. See field_folders.
        #[builder(into, default)]
        pub field_folders: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetFieldFolder>>,
        >,
        /// Indicates whether you want to import the data into SPICE. Valid values are `SPICE` and `DIRECT_QUERY`.
        #[builder(into)]
        pub import_mode: pulumi_wasm_rust::Output<String>,
        /// Configures the combination and transformation of the data from the physical tables. Maximum of 1 entry. See logical_table_map.
        #[builder(into, default)]
        pub logical_table_maps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetLogicalTableMap>>,
        >,
        /// Display name for the dataset.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of resource permissions on the data source. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetPermission>>,
        >,
        /// Declares the physical tables that are available in the underlying data sources. See physical_table_map.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub physical_table_maps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetPhysicalTableMap>>,
        >,
        /// The refresh properties for the data set. **NOTE**: Only valid when `import_mode` is set to `SPICE`. See refresh_properties.
        #[builder(into, default)]
        pub refresh_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSetRefreshProperties>,
        >,
        /// The row-level security configuration for the data that you want to create. See row_level_permission_data_set.
        #[builder(into, default)]
        pub row_level_permission_data_set: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSetRowLevelPermissionDataSet>,
        >,
        /// The configuration of tags on a dataset to set row-level security. Row-level security tags are currently supported for anonymous embedding only. See row_level_permission_tag_configuration.
        #[builder(into, default)]
        pub row_level_permission_tag_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::quicksight::DataSetRowLevelPermissionTagConfiguration,
            >,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataSetResult {
        /// Amazon Resource Name (ARN) of the data set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// Groupings of columns that work together in certain Amazon QuickSight features. Currently, only geospatial hierarchy is supported. See column_groups.
        pub column_groups: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetColumnGroup>>,
        >,
        /// A set of 1 or more definitions of a [ColumnLevelPermissionRule](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_ColumnLevelPermissionRule.html). See column_level_permission_rules.
        pub column_level_permission_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::quicksight::DataSetColumnLevelPermissionRule>,
            >,
        >,
        /// Identifier for the data set.
        pub data_set_id: pulumi_wasm_rust::Output<String>,
        /// The usage configuration to apply to child datasets that reference this dataset as a source. See data_set_usage_configuration.
        pub data_set_usage_configuration: pulumi_wasm_rust::Output<
            super::super::types::quicksight::DataSetDataSetUsageConfiguration,
        >,
        /// The folder that contains fields and nested subfolders for your dataset. See field_folders.
        pub field_folders: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetFieldFolder>>,
        >,
        /// Indicates whether you want to import the data into SPICE. Valid values are `SPICE` and `DIRECT_QUERY`.
        pub import_mode: pulumi_wasm_rust::Output<String>,
        /// Configures the combination and transformation of the data from the physical tables. Maximum of 1 entry. See logical_table_map.
        pub logical_table_maps: pulumi_wasm_rust::Output<
            Vec<super::super::types::quicksight::DataSetLogicalTableMap>,
        >,
        /// Display name for the dataset.
        pub name: pulumi_wasm_rust::Output<String>,
        pub output_columns: pulumi_wasm_rust::Output<
            Vec<super::super::types::quicksight::DataSetOutputColumn>,
        >,
        /// A set of resource permissions on the data source. Maximum of 64 items. See permissions.
        pub permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetPermission>>,
        >,
        /// Declares the physical tables that are available in the underlying data sources. See physical_table_map.
        ///
        /// The following arguments are optional:
        pub physical_table_maps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::quicksight::DataSetPhysicalTableMap>>,
        >,
        /// The refresh properties for the data set. **NOTE**: Only valid when `import_mode` is set to `SPICE`. See refresh_properties.
        pub refresh_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSetRefreshProperties>,
        >,
        /// The row-level security configuration for the data that you want to create. See row_level_permission_data_set.
        pub row_level_permission_data_set: pulumi_wasm_rust::Output<
            Option<super::super::types::quicksight::DataSetRowLevelPermissionDataSet>,
        >,
        /// The configuration of tags on a dataset to set row-level security. Row-level security tags are currently supported for anonymous embedding only. See row_level_permission_tag_configuration.
        pub row_level_permission_tag_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::quicksight::DataSetRowLevelPermissionTagConfiguration,
            >,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataSetArgs) -> DataSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let column_groups_binding = args.column_groups.get_inner();
        let column_level_permission_rules_binding = args
            .column_level_permission_rules
            .get_inner();
        let data_set_id_binding = args.data_set_id.get_inner();
        let data_set_usage_configuration_binding = args
            .data_set_usage_configuration
            .get_inner();
        let field_folders_binding = args.field_folders.get_inner();
        let import_mode_binding = args.import_mode.get_inner();
        let logical_table_maps_binding = args.logical_table_maps.get_inner();
        let name_binding = args.name.get_inner();
        let permissions_binding = args.permissions.get_inner();
        let physical_table_maps_binding = args.physical_table_maps.get_inner();
        let refresh_properties_binding = args.refresh_properties.get_inner();
        let row_level_permission_data_set_binding = args
            .row_level_permission_data_set
            .get_inner();
        let row_level_permission_tag_configuration_binding = args
            .row_level_permission_tag_configuration
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/dataSet:DataSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "columnGroups".into(),
                    value: &column_groups_binding,
                },
                register_interface::ObjectField {
                    name: "columnLevelPermissionRules".into(),
                    value: &column_level_permission_rules_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetId".into(),
                    value: &data_set_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataSetUsageConfiguration".into(),
                    value: &data_set_usage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "fieldFolders".into(),
                    value: &field_folders_binding,
                },
                register_interface::ObjectField {
                    name: "importMode".into(),
                    value: &import_mode_binding,
                },
                register_interface::ObjectField {
                    name: "logicalTableMaps".into(),
                    value: &logical_table_maps_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "physicalTableMaps".into(),
                    value: &physical_table_maps_binding,
                },
                register_interface::ObjectField {
                    name: "refreshProperties".into(),
                    value: &refresh_properties_binding,
                },
                register_interface::ObjectField {
                    name: "rowLevelPermissionDataSet".into(),
                    value: &row_level_permission_data_set_binding,
                },
                register_interface::ObjectField {
                    name: "rowLevelPermissionTagConfiguration".into(),
                    value: &row_level_permission_tag_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "dataSetUsageConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "fieldFolders".into(),
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
                    name: "outputColumns".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "physicalTableMaps".into(),
                },
                register_interface::ResultField {
                    name: "refreshProperties".into(),
                },
                register_interface::ResultField {
                    name: "rowLevelPermissionDataSet".into(),
                },
                register_interface::ResultField {
                    name: "rowLevelPermissionTagConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DataSetResult {
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
            data_set_usage_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSetUsageConfiguration").unwrap(),
            ),
            field_folders: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fieldFolders").unwrap(),
            ),
            import_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importMode").unwrap(),
            ),
            logical_table_maps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicalTableMaps").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            output_columns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputColumns").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            physical_table_maps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("physicalTableMaps").unwrap(),
            ),
            refresh_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("refreshProperties").unwrap(),
            ),
            row_level_permission_data_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowLevelPermissionDataSet").unwrap(),
            ),
            row_level_permission_tag_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowLevelPermissionTagConfiguration").unwrap(),
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
