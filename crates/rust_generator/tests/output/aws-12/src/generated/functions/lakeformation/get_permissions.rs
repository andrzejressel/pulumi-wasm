pub mod get_permissions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionsArgs {
        /// Identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the permissions are to be granted for the Data Catalog. Defaults to `false`.
        #[builder(into, default)]
        pub catalog_resource: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block for a data cells filter resource. Detailed below.
        #[builder(into, default)]
        pub data_cells_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::super::types::lakeformation::GetPermissionsDataCellsFilter,
            >,
        >,
        /// Configuration block for a data location resource. Detailed below.
        #[builder(into, default)]
        pub data_location: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::lakeformation::GetPermissionsDataLocation>,
        >,
        /// Configuration block for a database resource. Detailed below.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::lakeformation::GetPermissionsDatabase>,
        >,
        /// Configuration block for an LF-tag resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::lakeformation::GetPermissionsLfTag>,
        >,
        /// Configuration block for an LF-tag policy resource. Detailed below.
        #[builder(into, default)]
        pub lf_tag_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::lakeformation::GetPermissionsLfTagPolicy>,
        >,
        /// Principal to be granted the permissions on the resource. Supported principals are IAM users or IAM roles.
        ///
        /// One of the following is required:
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for a table resource. Detailed below.
        #[builder(into, default)]
        pub table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::lakeformation::GetPermissionsTable>,
        >,
        /// Configuration block for a table with columns resource. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub table_with_columns: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::super::types::lakeformation::GetPermissionsTableWithColumns,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPermissionsResult {
        pub catalog_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub catalog_resource: pulumi_gestalt_rust::Output<Option<bool>>,
        pub data_cells_filter: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDataCellsFilter,
        >,
        pub data_location: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDataLocation,
        >,
        pub database: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsDatabase,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub lf_tag: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsLfTag,
        >,
        pub lf_tag_policy: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsLfTagPolicy,
        >,
        /// List of permissions granted to the principal. For details on permissions, see [Lake Formation Permissions Reference](https://docs.aws.amazon.com/lake-formation/latest/dg/lf-permissions-reference.html).
        pub permissions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Subset of `permissions` which the principal can pass.
        pub permissions_with_grant_options: pulumi_gestalt_rust::Output<Vec<String>>,
        pub principal: pulumi_gestalt_rust::Output<String>,
        pub table: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsTable,
        >,
        pub table_with_columns: pulumi_gestalt_rust::Output<
            super::super::super::types::lakeformation::GetPermissionsTableWithColumns,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetPermissionsArgs,
    ) -> GetPermissionsResult {
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
        let principal_binding = args.principal.get_output(context).get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let table_with_columns_binding = args
            .table_with_columns
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPermissionsResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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
