/// Provides an RDS DB option group resource. Documentation of the available options for various RDS engines can be found at:
///
/// * [MariaDB Options](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.MariaDB.Options.html)
/// * [Microsoft SQL Server Options](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.SQLServer.Options.html)
/// * [MySQL Options](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.MySQL.Options.html)
/// * [Oracle Options](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.Oracle.Options.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = option_group::create(
///         "example",
///         OptionGroupArgs::builder()
///             .engine_name("sqlserver-ee")
///             .major_engine_version("11.00")
///             .name("option-group-test")
///             .option_group_description("Option Group")
///             .options(
///                 vec![
///                     OptionGroupOption::builder().optionName("Timezone")
///                     .optionSettings(vec![OptionGroupOptionOptionSetting::builder()
///                     .name("TIME_ZONE").value("UTC").build_struct(),]).build_struct(),
///                     OptionGroupOption::builder().optionName("SQLSERVER_BACKUP_RESTORE")
///                     .optionSettings(vec![OptionGroupOptionOptionSetting::builder()
///                     .name("IAM_ROLE_ARN").value("${exampleAwsIamRole.arn}")
///                     .build_struct(),]).build_struct(), OptionGroupOption::builder()
///                     .optionName("TDE").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** Any modifications to the `aws.rds.OptionGroup` are set to happen immediately as we default to applying immediately.
///
/// > **WARNING:** You can perform a destroy on a `aws.rds.OptionGroup`, as long as it is not associated with any Amazon RDS resource. An option group can be associated with a DB instance, a manual DB snapshot, or an automated DB snapshot.
///
/// If you try to delete an option group that is associated with an Amazon RDS resource, an error similar to the following is returned:
///
/// > An error occurred (InvalidOptionGroupStateFault) when calling the DeleteOptionGroup operation: The option group 'optionGroupName' cannot be deleted because it is in use.
///
/// More information about this can be found [here](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithOptionGroups.html#USER_WorkingWithOptionGroups.Delete).
///
/// ## Import
///
/// Using `pulumi import`, import DB option groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/optionGroup:OptionGroup example mysql-option-group
/// ```
pub mod option_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OptionGroupArgs {
        /// Specifies the name of the engine that this option group should be associated with.
        #[builder(into)]
        pub engine_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the major version of the engine that this option group should be associated with.
        #[builder(into)]
        pub major_engine_version: pulumi_wasm_rust::Output<String>,
        /// Name of the option group. If omitted, the provider will assign a random, unique name. Must be lowercase, to match as it is stored in AWS.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Must be lowercase, to match as it is stored in AWS.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the option group. Defaults to "Managed by Pulumi".
        #[builder(into, default)]
        pub option_group_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The options to apply. See `option` Block below for more details.
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::rds::OptionGroupOption>>,
        >,
        /// Set to true if you do not wish the option group to be deleted at destroy time, and instead just remove the option group from the Pulumi state.
        #[builder(into, default)]
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct OptionGroupResult {
        /// ARN of the DB option group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the engine that this option group should be associated with.
        pub engine_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the major version of the engine that this option group should be associated with.
        pub major_engine_version: pulumi_wasm_rust::Output<String>,
        /// Name of the option group. If omitted, the provider will assign a random, unique name. Must be lowercase, to match as it is stored in AWS.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`. Must be lowercase, to match as it is stored in AWS.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Description of the option group. Defaults to "Managed by Pulumi".
        pub option_group_description: pulumi_wasm_rust::Output<String>,
        /// The options to apply. See `option` Block below for more details.
        pub options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::rds::OptionGroupOption>>,
        >,
        /// Set to true if you do not wish the option group to be deleted at destroy time, and instead just remove the option group from the Pulumi state.
        pub skip_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OptionGroupArgs) -> OptionGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_name_binding = args.engine_name.get_inner();
        let major_engine_version_binding = args.major_engine_version.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let option_group_description_binding = args.option_group_description.get_inner();
        let options_binding = args.options.get_inner();
        let skip_destroy_binding = args.skip_destroy.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/optionGroup:OptionGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engineName".into(),
                    value: &engine_name_binding,
                },
                register_interface::ObjectField {
                    name: "majorEngineVersion".into(),
                    value: &major_engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "optionGroupDescription".into(),
                    value: &option_group_description_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
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
                    name: "engineName".into(),
                },
                register_interface::ResultField {
                    name: "majorEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "optionGroupDescription".into(),
                },
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "skipDestroy".into(),
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
        OptionGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            engine_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineName").unwrap(),
            ),
            major_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("majorEngineVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            option_group_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optionGroupDescription").unwrap(),
            ),
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            skip_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipDestroy").unwrap(),
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
