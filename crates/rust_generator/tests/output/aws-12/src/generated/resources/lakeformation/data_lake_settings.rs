/// Manages Lake Formation principals designated as data lake administrators and lists of principal permission entries for default create database and default create table permissions.
///
/// > **NOTE:** Lake Formation introduces fine-grained access control for data in your data lake. Part of the changes include the `IAMAllowedPrincipals` principal in order to make Lake Formation backwards compatible with existing IAM and Glue permissions. For more information, see [Changing the Default Security Settings for Your Data Lake](https://docs.aws.amazon.com/lake-formation/latest/dg/change-settings.html) and [Upgrading AWS Glue Data Permissions to the AWS Lake Formation Model](https://docs.aws.amazon.com/lake-formation/latest/dg/upgrade-glue-lake-formation.html).
///
/// ## Example Usage
///
/// ### Data Lake Admins
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_lake_settings::create(
///         "example",
///         DataLakeSettingsArgs::builder()
///             .admins(vec!["${test.arn}", "${testAwsIamRole.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create Default Permissions
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_lake_settings::create(
///         "example",
///         DataLakeSettingsArgs::builder()
///             .admins(vec!["${test.arn}", "${testAwsIamRole.arn}",])
///             .create_database_default_permissions(
///                 vec![
///                     DataLakeSettingsCreateDatabaseDefaultPermission::builder()
///                     .permissions(vec!["SELECT", "ALTER", "DROP",])
///                     .principal("${test.arn}").build_struct(),
///                 ],
///             )
///             .create_table_default_permissions(
///                 vec![
///                     DataLakeSettingsCreateTableDefaultPermission::builder()
///                     .permissions(vec!["ALL",]).principal("${testAwsIamRole.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enable EMR access to LakeFormation resources
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = data_lake_settings::create(
///         "example",
///         DataLakeSettingsArgs::builder()
///             .admins(vec!["${test.arn}", "${testAwsIamRole.arn}",])
///             .allow_external_data_filtering(true)
///             .allow_full_table_external_data_access(true)
///             .authorized_session_tag_value_lists(vec!["Amazon EMR",])
///             .create_database_default_permissions(
///                 vec![
///                     DataLakeSettingsCreateDatabaseDefaultPermission::builder()
///                     .permissions(vec!["SELECT", "ALTER", "DROP",])
///                     .principal("${test.arn}").build_struct(),
///                 ],
///             )
///             .create_table_default_permissions(
///                 vec![
///                     DataLakeSettingsCreateTableDefaultPermission::builder()
///                     .permissions(vec!["ALL",]).principal("${testAwsIamRole.arn}")
///                     .build_struct(),
///                 ],
///             )
///             .external_data_filtering_allow_lists(
///                 vec!["${current.accountId}", "${thirdParty.accountId}",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Change Cross Account Version
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lakeformation:DataLakeSettings
///     properties:
///       parameters:
///         CROSS_ACCOUNT_VERSION: '3'
/// ```
pub mod data_lake_settings {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataLakeSettingsArgs {
        /// Set of ARNs of AWS Lake Formation principals (IAM users or roles).
        #[builder(into, default)]
        pub admins: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to allow Amazon EMR clusters to access data managed by Lake Formation.
        #[builder(into, default)]
        pub allow_external_data_filtering: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to allow a third-party query engine to get data access credentials without session tags when a caller has full data access permissions.
        #[builder(into, default)]
        pub allow_full_table_external_data_access: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Lake Formation relies on a privileged process secured by Amazon EMR or the third party integrator to tag the user's role while assuming it.
        #[builder(into, default)]
        pub authorized_session_tag_value_lists: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Identifier for the Data Catalog. By default, the account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Up to three configuration blocks of principal permissions for default create database permissions. Detailed below.
        #[builder(into, default)]
        pub create_database_default_permissions: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::lakeformation::DataLakeSettingsCreateDatabaseDefaultPermission,
                >,
            >,
        >,
        /// Up to three configuration blocks of principal permissions for default create table permissions. Detailed below.
        #[builder(into, default)]
        pub create_table_default_permissions: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::lakeformation::DataLakeSettingsCreateTableDefaultPermission,
                >,
            >,
        >,
        /// A list of the account IDs of Amazon Web Services accounts with Amazon EMR clusters that are to perform data filtering.
        #[builder(into, default)]
        pub external_data_filtering_allow_lists: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Key-value map of additional configuration. Valid values for the `CROSS_ACCOUNT_VERSION` key are `"1"`, `"2"`, `"3"`, or `"4"`. `SET_CONTEXT` is also returned with a value of `TRUE`. In a fresh account, prior to configuring, `CROSS_ACCOUNT_VERSION` is `"1"`. Destroying this resource sets the `CROSS_ACCOUNT_VERSION` to `"1"`.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of ARNs of AWS Lake Formation principals (IAM users or roles) with only view access to the resources.
        #[builder(into, default)]
        pub read_only_admins: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs).
        ///
        /// > **NOTE:** Although optional, not including `admins`, `create_database_default_permissions`, `create_table_default_permissions`, `parameters`, and/or `trusted_resource_owners` results in the setting being cleared.
        #[builder(into, default)]
        pub trusted_resource_owners: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataLakeSettingsResult {
        /// Set of ARNs of AWS Lake Formation principals (IAM users or roles).
        pub admins: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether to allow Amazon EMR clusters to access data managed by Lake Formation.
        pub allow_external_data_filtering: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to allow a third-party query engine to get data access credentials without session tags when a caller has full data access permissions.
        pub allow_full_table_external_data_access: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Lake Formation relies on a privileged process secured by Amazon EMR or the third party integrator to tag the user's role while assuming it.
        pub authorized_session_tag_value_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// Identifier for the Data Catalog. By default, the account ID.
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Up to three configuration blocks of principal permissions for default create database permissions. Detailed below.
        pub create_database_default_permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::lakeformation::DataLakeSettingsCreateDatabaseDefaultPermission,
            >,
        >,
        /// Up to three configuration blocks of principal permissions for default create table permissions. Detailed below.
        pub create_table_default_permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::lakeformation::DataLakeSettingsCreateTableDefaultPermission,
            >,
        >,
        /// A list of the account IDs of Amazon Web Services accounts with Amazon EMR clusters that are to perform data filtering.
        pub external_data_filtering_allow_lists: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of additional configuration. Valid values for the `CROSS_ACCOUNT_VERSION` key are `"1"`, `"2"`, `"3"`, or `"4"`. `SET_CONTEXT` is also returned with a value of `TRUE`. In a fresh account, prior to configuring, `CROSS_ACCOUNT_VERSION` is `"1"`. Destroying this resource sets the `CROSS_ACCOUNT_VERSION` to `"1"`.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of ARNs of AWS Lake Formation principals (IAM users or roles) with only view access to the resources.
        pub read_only_admins: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs).
        ///
        /// > **NOTE:** Although optional, not including `admins`, `create_database_default_permissions`, `create_table_default_permissions`, `parameters`, and/or `trusted_resource_owners` results in the setting being cleared.
        pub trusted_resource_owners: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DataLakeSettingsArgs,
    ) -> DataLakeSettingsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admins_binding = args.admins.get_output(context).get_inner();
        let allow_external_data_filtering_binding = args
            .allow_external_data_filtering
            .get_output(context)
            .get_inner();
        let allow_full_table_external_data_access_binding = args
            .allow_full_table_external_data_access
            .get_output(context)
            .get_inner();
        let authorized_session_tag_value_lists_binding = args
            .authorized_session_tag_value_lists
            .get_output(context)
            .get_inner();
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let create_database_default_permissions_binding = args
            .create_database_default_permissions
            .get_output(context)
            .get_inner();
        let create_table_default_permissions_binding = args
            .create_table_default_permissions
            .get_output(context)
            .get_inner();
        let external_data_filtering_allow_lists_binding = args
            .external_data_filtering_allow_lists
            .get_output(context)
            .get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let read_only_admins_binding = args
            .read_only_admins
            .get_output(context)
            .get_inner();
        let trusted_resource_owners_binding = args
            .trusted_resource_owners
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/dataLakeSettings:DataLakeSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "admins".into(),
                    value: &admins_binding,
                },
                register_interface::ObjectField {
                    name: "allowExternalDataFiltering".into(),
                    value: &allow_external_data_filtering_binding,
                },
                register_interface::ObjectField {
                    name: "allowFullTableExternalDataAccess".into(),
                    value: &allow_full_table_external_data_access_binding,
                },
                register_interface::ObjectField {
                    name: "authorizedSessionTagValueLists".into(),
                    value: &authorized_session_tag_value_lists_binding,
                },
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "createDatabaseDefaultPermissions".into(),
                    value: &create_database_default_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "createTableDefaultPermissions".into(),
                    value: &create_table_default_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "externalDataFilteringAllowLists".into(),
                    value: &external_data_filtering_allow_lists_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "readOnlyAdmins".into(),
                    value: &read_only_admins_binding,
                },
                register_interface::ObjectField {
                    name: "trustedResourceOwners".into(),
                    value: &trusted_resource_owners_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataLakeSettingsResult {
            admins: pulumi_wasm_rust::__private::into_domain(o.extract_field("admins")),
            allow_external_data_filtering: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowExternalDataFiltering"),
            ),
            allow_full_table_external_data_access: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowFullTableExternalDataAccess"),
            ),
            authorized_session_tag_value_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizedSessionTagValueLists"),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            create_database_default_permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createDatabaseDefaultPermissions"),
            ),
            create_table_default_permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTableDefaultPermissions"),
            ),
            external_data_filtering_allow_lists: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("externalDataFilteringAllowLists"),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            read_only_admins: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readOnlyAdmins"),
            ),
            trusted_resource_owners: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustedResourceOwners"),
            ),
        }
    }
}
