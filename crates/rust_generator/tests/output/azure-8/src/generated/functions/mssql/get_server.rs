#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerArgs {
        /// The name of this Microsoft SQL Server.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Microsoft SQL Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerResult {
        /// The server's administrator login name.
        pub administrator_login: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified domain name of the Azure SQL Server.
        pub fully_qualified_domain_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mssql::GetServerIdentity>,
        >,
        /// The Azure Region where the Microsoft SQL Server exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of dropped restorable database IDs on the server.
        pub restorable_dropped_database_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags assigned to this Microsoft SQL Server.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Key Vault key URI to be used as the `Customer Managed Key`(CMK/BYOK) for the `Transparent Data Encryption`(TDE) layer.
        pub transparent_data_encryption_key_vault_key_id: pulumi_gestalt_rust::Output<
            String,
        >,
        /// This servers MS SQL version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServerArgs,
    ) -> GetServerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mssql/getServer:getServer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServerResult {
            administrator_login: o.get_field("administratorLogin"),
            fully_qualified_domain_name: o.get_field("fullyQualifiedDomainName"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            restorable_dropped_database_ids: o.get_field("restorableDroppedDatabaseIds"),
            tags: o.get_field("tags"),
            transparent_data_encryption_key_vault_key_id: o
                .get_field("transparentDataEncryptionKeyVaultKeyId"),
            version: o.get_field("version"),
        }
    }
}
