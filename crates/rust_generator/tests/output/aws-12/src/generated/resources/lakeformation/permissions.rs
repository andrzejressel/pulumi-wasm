/// Grants permissions to the principal to access metadata in the Data Catalog and data organized in underlying data storage such as Amazon S3. Permissions are granted to a principal, in a Data Catalog, relative to a Lake Formation resource, which includes the Data Catalog, databases, tables, LF-tags, and LF-tag policies. For more information, see [Security and Access Control to Metadata and Data in Lake Formation](https://docs.aws.amazon.com/lake-formation/latest/dg/security-data-access.html).
///
/// !> **WARNING:** Lake Formation permissions are not in effect by default within AWS. Using this resource will not secure your data and will result in errors if you do not change the security settings for existing resources and the default security settings for new resources. See Default Behavior and `IAMAllowedPrincipals` for additional details.
///
/// > **NOTE:** In general, the `principal` should _NOT_ be a Lake Formation administrator or the entity (e.g., IAM role) that is running the deployment. Administrators have implicit permissions. These should be managed by granting or not granting administrator rights using `aws.lakeformation.DataLakeSettings`, _not_ with this resource.
///
/// ## Default Behavior and `IAMAllowedPrincipals`
///
/// **_Lake Formation permissions are not in effect by default within AWS._** `IAMAllowedPrincipals` (i.e., `IAM_ALLOWED_PRINCIPALS`) conflicts with individual Lake Formation permissions (i.e., non-`IAMAllowedPrincipals` permissions), will cause unexpected behavior, and may result in errors.
///
/// When using Lake Formation, choose ONE of the following options as they are mutually exclusive:
///
/// 1. Use this resource (`aws.lakeformation.Permissions`), change the default security settings using `aws.lakeformation.DataLakeSettings`, and remove existing `IAMAllowedPrincipals` permissions
/// 2. Use `IAMAllowedPrincipals` without `aws.lakeformation.Permissions`
///
/// This example shows removing the `IAMAllowedPrincipals` default security settings and making the caller a Lake Formation admin. Since `create_database_default_permissions` and `create_table_default_permissions` are not set in the `aws.lakeformation.DataLakeSettings` resource, they are cleared.
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lakeformation:DataLakeSettings
///     properties:
///       admins:
///         - ${currentGetSessionContext.issuerArn}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetSessionContext:
///     fn::invoke:
///       function: aws:iam:getSessionContext
///       arguments:
///         arn: ${current.arn}
/// ```
///
/// To remove existing `IAMAllowedPrincipals` permissions, use the [AWS Lake Formation Console](https://console.aws.amazon.com/lakeformation/) or [AWS CLI](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lakeformation/batch-revoke-permissions.html).
///
/// `IAMAllowedPrincipals` is a hook to maintain backwards compatibility with AWS Glue. `IAMAllowedPrincipals` is a pseudo-entity group that acts like a Lake Formation principal. The group includes any IAM users and roles that are allowed access to your Data Catalog resources by your IAM policies.
///
/// This is Lake Formation's default behavior:
///
/// * Lake Formation grants `Super` permission to `IAMAllowedPrincipals` on all existing AWS Glue Data Catalog resources.
/// * Lake Formation enables "Use only IAM access control" for new Data Catalog resources.
///
/// For more details, see [Changing the Default Security Settings for Your Data Lake](https://docs.aws.amazon.com/lake-formation/latest/dg/change-settings.html).
///
/// ### Problem Using `IAMAllowedPrincipals`
///
/// AWS does not support combining `IAMAllowedPrincipals` permissions and non-`IAMAllowedPrincipals` permissions. Doing so results in unexpected permissions and behaviors. For example, this configuration grants a user `SELECT` on a column in a table.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = catalog_database::create(
///         "example",
///         CatalogDatabaseArgs::builder().name("sadabate").build_struct(),
///     );
///     let exampleCatalogTable = catalog_table::create(
///         "exampleCatalogTable",
///         CatalogTableArgs::builder()
///             .database_name("${test.name}")
///             .name("abelt")
///             .storage_descriptor(
///                 CatalogTableStorageDescriptor::builder()
///                     .columns(
///                         vec![
///                             CatalogTableStorageDescriptorColumn::builder().name("event").
///                             type ("string").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let examplePermissions = permissions::create(
///         "examplePermissions",
///         PermissionsArgs::builder()
///             .permissions(vec!["SELECT",])
///             .principal("arn:aws:iam:us-east-1:123456789012:user/SanHolo")
///             .table_with_columns(
///                 PermissionsTableWithColumns::builder()
///                     .columnNames(vec!["event",])
///                     .databaseName("${exampleCatalogTable.databaseName}")
///                     .name("${exampleCatalogTable.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// The resulting permissions depend on whether the table had `IAMAllowedPrincipals` (IAP) permissions or not.
///
/// | Result With IAP | Result Without IAP |
/// | ---- | ---- |
/// | `SELECT` column wildcard (i.e., all columns) | `SELECT` on `"event"` (as expected) |
///
/// ## `ALLIAMPrincipals` group
///
/// AllIAMPrincipals is a pseudo-entity group that acts like a Lake Formation principal. The group includes all IAMs in the account that is defined.
///
/// resource "aws.lakeformation.Permissions" "example" {
///   permissions = ["SELECT"]
///   principal   = "123456789012:IAMPrincipals"
///
///   table_with_columns {
///     database_name = aws_glue_catalog_table.example.database_name
///     name          = aws_glue_catalog_table.example.name
///     column_names  = ["event"]
///   }
/// }
///
/// ## Using Lake Formation Permissions
///
/// Lake Formation grants implicit permissions to data lake administrators, database creators, and table creators. These implicit permissions cannot be revoked _per se_. If this resource reads implicit permissions, it will attempt to revoke them, which causes an error when the resource is destroyed.
///
/// There are two ways to avoid these errors. First, and the way we recommend, is to avoid using this resource with principals that have implicit permissions. A second, error-prone option, is to grant explicit permissions (and `permissions_with_grant_option`) to "overwrite" a principal's implicit permissions, which you can then revoke with this resource. For more information, see [Implicit Lake Formation Permissions](https://docs.aws.amazon.com/lake-formation/latest/dg/implicit-permissions.html).
///
/// If the `principal` is also a data lake administrator, AWS grants implicit permissions that can cause errors using this resource. For example, AWS implicitly grants a `principal`/administrator `permissions` and `permissions_with_grant_option` of `ALL`, `ALTER`, `DELETE`, `DESCRIBE`, `DROP`, `INSERT`, and `SELECT` on a table. If you use this resource to explicitly grant the `principal`/administrator `permissions` but _not_ `permissions_with_grant_option` of `ALL`, `ALTER`, `DELETE`, `DESCRIBE`, `DROP`, `INSERT`, and `SELECT` on the table, this resource will read the implicit `permissions_with_grant_option` and attempt to revoke them when the resource is destroyed. Doing so will cause an `InvalidInputException: No permissions revoked` error because you cannot revoke implicit permissions _per se_. To workaround this problem, explicitly grant the `principal`/administrator `permissions` _and_ `permissions_with_grant_option`, which can then be revoked. Similarly, granting a `principal`/administrator permissions on a table with columns and providing `column_names`, will result in a `InvalidInputException: Permissions modification is invalid` error because you are narrowing the implicit permissions. Instead, set `wildcard` to `true` and remove the `column_names`.
///
/// ## Example Usage
///
/// ### Grant Permissions For A Lake Formation S3 Resource
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = permissions::create(
///         "example",
///         PermissionsArgs::builder()
///             .data_location(
///                 PermissionsDataLocation::builder()
///                     .arn("${exampleAwsLakeformationResource.arn}")
///                     .build_struct(),
///             )
///             .permissions(vec!["DATA_LOCATION_ACCESS",])
///             .principal("${workflowRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Grant Permissions For A Glue Catalog Database
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = permissions::create(
///         "example",
///         PermissionsArgs::builder()
///             .database(
///                 PermissionsDatabase::builder()
///                     .catalogId("110376042874")
///                     .name("${exampleAwsGlueCatalogDatabase.name}")
///                     .build_struct(),
///             )
///             .permissions(vec!["CREATE_TABLE", "ALTER", "DROP",])
///             .principal("${workflowRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Grant Permissions Using Tag-Based Access Control
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = permissions::create(
///         "test",
///         PermissionsArgs::builder()
///             .lf_tag_policy(
///                 PermissionsLfTagPolicy::builder()
///                     .expressions(
///                         vec![
///                             PermissionsLfTagPolicyExpression::builder().key("Team")
///                             .values(vec!["Sales",]).build_struct(),
///                             PermissionsLfTagPolicyExpression::builder()
///                             .key("Environment").values(vec!["Dev", "Production",])
///                             .build_struct(),
///                         ],
///                     )
///                     .resourceType("DATABASE")
///                     .build_struct(),
///             )
///             .permissions(vec!["CREATE_TABLE", "ALTER", "DROP",])
///             .principal("${salesRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod permissions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionsArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the permissions are to be granted for the Data Catalog. Defaults to `false`.
        #[builder(into, default)]
        pub catalog_resource: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for a data cells filter resource. Detailed below.
        #[builder(into, default)]
        pub data_cells_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsDataCellsFilter>,
        >,
        /// Configuration block for a data location resource. Detailed below.
        #[builder(into, default)]
        pub data_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsDataLocation>,
        >,
        /// Configuration block for a database resource. Detailed below.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsDatabase>,
        >,
        /// Configuration block for an LF-tag resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsLfTag>,
        >,
        /// Configuration block for an LF-tag policy resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsLfTagPolicy>,
        >,
        /// List of permissions granted to the principal. Valid values may include `ALL`, `ALTER`, `ASSOCIATE`, `CREATE_DATABASE`, `CREATE_TABLE`, `DATA_LOCATION_ACCESS`, `DELETE`, `DESCRIBE`, `DROP`, `INSERT`, and `SELECT`. For details on each permission, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Subset of `permissions` which the principal can pass.
        #[builder(into, default)]
        pub permissions_with_grant_options: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Principal to be granted the permissions on the resource. Supported principals include `IAM_ALLOWED_PRINCIPALS` (see Default Behavior and `IAMAllowedPrincipals` above), IAM roles, users, groups, Federated Users, SAML groups and users, QuickSight groups, OUs, and organizations as well as AWS account IDs for cross-account permissions. For more information, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        ///
        /// > **NOTE:** We highly recommend that the `principal` _NOT_ be a Lake Formation administrator (granted using `aws.lakeformation.DataLakeSettings`). The entity (e.g., IAM role) running the deployment will most likely need to be a Lake Formation administrator. As such, the entity will have implicit permissions and does not need permissions granted through this resource.
        ///
        /// One of the following is required:
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for a table resource. Detailed below.
        #[builder(into, default)]
        pub table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsTable>,
        >,
        /// Configuration block for a table with columns resource. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lakeformation::PermissionsTableWithColumns>,
        >,
    }
    #[allow(dead_code)]
    pub struct PermissionsResult {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        pub catalog_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the permissions are to be granted for the Data Catalog. Defaults to `false`.
        pub catalog_resource: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Configuration block for a data cells filter resource. Detailed below.
        pub data_cells_filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::lakeformation::PermissionsDataCellsFilter>,
        >,
        /// Configuration block for a data location resource. Detailed below.
        pub data_location: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsDataLocation,
        >,
        /// Configuration block for a database resource. Detailed below.
        pub database: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsDatabase,
        >,
        /// Configuration block for an LF-tag resource. Detailed below.
        pub lf_tag: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsLfTag,
        >,
        /// Configuration block for an LF-tag policy resource. Detailed below.
        pub lf_tag_policy: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsLfTagPolicy,
        >,
        /// List of permissions granted to the principal. Valid values may include `ALL`, `ALTER`, `ASSOCIATE`, `CREATE_DATABASE`, `CREATE_TABLE`, `DATA_LOCATION_ACCESS`, `DELETE`, `DESCRIBE`, `DROP`, `INSERT`, and `SELECT`. For details on each permission, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        pub permissions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subset of `permissions` which the principal can pass.
        pub permissions_with_grant_options: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Principal to be granted the permissions on the resource. Supported principals include `IAM_ALLOWED_PRINCIPALS` (see Default Behavior and `IAMAllowedPrincipals` above), IAM roles, users, groups, Federated Users, SAML groups and users, QuickSight groups, OUs, and organizations as well as AWS account IDs for cross-account permissions. For more information, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        ///
        /// > **NOTE:** We highly recommend that the `principal` _NOT_ be a Lake Formation administrator (granted using `aws.lakeformation.DataLakeSettings`). The entity (e.g., IAM role) running the deployment will most likely need to be a Lake Formation administrator. As such, the entity will have implicit permissions and does not need permissions granted through this resource.
        ///
        /// One of the following is required:
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for a table resource. Detailed below.
        pub table: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsTable,
        >,
        /// Configuration block for a table with columns resource. Detailed below.
        ///
        /// The following arguments are optional:
        pub table_with_columns: pulumi_gestalt_rust::Output<
            super::super::types::lakeformation::PermissionsTableWithColumns,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PermissionsArgs,
    ) -> PermissionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let catalog_resource_binding = args
            .catalog_resource
            .get_output(context)
            .get_inner();
        let data_cells_filter_binding = args
            .data_cells_filter
            .get_output(context)
            .get_inner();
        let data_location_binding = args.data_location.get_output(context).get_inner();
        let database_binding = args.database.get_output(context).get_inner();
        let lf_tag_binding = args.lf_tag.get_output(context).get_inner();
        let lf_tag_policy_binding = args.lf_tag_policy.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let permissions_with_grant_options_binding = args
            .permissions_with_grant_options
            .get_output(context)
            .get_inner();
        let principal_binding = args.principal.get_output(context).get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let table_with_columns_binding = args
            .table_with_columns
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/permissions:Permissions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "permissionsWithGrantOptions".into(),
                    value: &permissions_with_grant_options_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PermissionsResult {
            catalog_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            catalog_resource: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogResource"),
            ),
            data_cells_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataCellsFilter"),
            ),
            data_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataLocation"),
            ),
            database: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            lf_tag: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lfTag"),
            ),
            lf_tag_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lfTagPolicy"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            permissions_with_grant_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissionsWithGrantOptions"),
            ),
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            table: pulumi_gestalt_rust::__private::into_domain(o.extract_field("table")),
            table_with_columns: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableWithColumns"),
            ),
        }
    }
}
