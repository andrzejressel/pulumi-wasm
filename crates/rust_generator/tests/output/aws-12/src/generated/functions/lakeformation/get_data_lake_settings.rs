#[allow(clippy::doc_lazy_continuation)]
pub mod get_data_lake_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataLakeSettingsArgs {
        /// Identifier for the Data Catalog. By default, the account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDataLakeSettingsResult {
        /// List of ARNs of AWS Lake Formation principals (IAM users or roles).
        pub admins: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether to allow Amazon EMR clusters to access data managed by Lake Formation.
        pub allow_external_data_filtering: pulumi_gestalt_rust::Output<bool>,
        /// Whether to allow a third-party query engine to get data access credentials without session tags when a caller has full data access permissions.
        pub allow_full_table_external_data_access: pulumi_gestalt_rust::Output<bool>,
        /// Lake Formation relies on a privileged process secured by Amazon EMR or the third party integrator to tag the user's role while assuming it.
        pub authorized_session_tag_value_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        pub catalog_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Up to three configuration blocks of principal permissions for default create database permissions. Detailed below.
        pub create_database_default_permissions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::lakeformation::GetDataLakeSettingsCreateDatabaseDefaultPermission,
            >,
        >,
        /// Up to three configuration blocks of principal permissions for default create table permissions. Detailed below.
        pub create_table_default_permissions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::lakeformation::GetDataLakeSettingsCreateTableDefaultPermission,
            >,
        >,
        /// A list of the account IDs of Amazon Web Services accounts with Amazon EMR clusters that are to perform data filtering.
        pub external_data_filtering_allow_lists: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of additional configuration. `CROSS_ACCOUNT_VERSION` will be set to values `"1"`, `"2"`, `"3"`, or `"4"`. `SET_CONTEXT` will also be returned with a value of `TRUE`. In a fresh account, prior to configuring, `CROSS_ACCOUNT_VERSION` is `"1"`.
        pub parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// List of ARNs of AWS Lake Formation principals (IAM users or roles) with only view access to the resources.
        pub read_only_admins: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs).
        pub trusted_resource_owners: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDataLakeSettingsArgs,
    ) -> GetDataLakeSettingsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lakeformation/getDataLakeSettings:getDataLakeSettings".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDataLakeSettingsResult {
            admins: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("admins"),
            ),
            allow_external_data_filtering: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowExternalDataFiltering"),
            ),
            allow_full_table_external_data_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowFullTableExternalDataAccess"),
            ),
            authorized_session_tag_value_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizedSessionTagValueLists"),
            ),
            catalog_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            create_database_default_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createDatabaseDefaultPermissions"),
            ),
            create_table_default_permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTableDefaultPermissions"),
            ),
            external_data_filtering_allow_lists: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalDataFilteringAllowLists"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            read_only_admins: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readOnlyAdmins"),
            ),
            trusted_resource_owners: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustedResourceOwners"),
            ),
        }
    }
}
