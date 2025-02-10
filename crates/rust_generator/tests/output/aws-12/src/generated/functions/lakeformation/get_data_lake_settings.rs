#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetDataLakeSettingsArgs,
    ) -> GetDataLakeSettingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_id_binding = args.catalog_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:lakeformation/getDataLakeSettings:getDataLakeSettings".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogId".into(),
                    value: catalog_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDataLakeSettingsResult {
            admins: o.get_field("admins"),
            allow_external_data_filtering: o.get_field("allowExternalDataFiltering"),
            allow_full_table_external_data_access: o
                .get_field("allowFullTableExternalDataAccess"),
            authorized_session_tag_value_lists: o
                .get_field("authorizedSessionTagValueLists"),
            catalog_id: o.get_field("catalogId"),
            create_database_default_permissions: o
                .get_field("createDatabaseDefaultPermissions"),
            create_table_default_permissions: o
                .get_field("createTableDefaultPermissions"),
            external_data_filtering_allow_lists: o
                .get_field("externalDataFilteringAllowLists"),
            id: o.get_field("id"),
            parameters: o.get_field("parameters"),
            read_only_admins: o.get_field("readOnlyAdmins"),
            trusted_resource_owners: o.get_field("trustedResourceOwners"),
        }
    }
}
