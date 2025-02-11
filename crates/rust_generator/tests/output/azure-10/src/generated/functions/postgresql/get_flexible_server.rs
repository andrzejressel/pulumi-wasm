#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_flexible_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFlexibleServerArgs {
        /// The name of this PostgreSQL Flexible Server.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the PostgreSQL Flexible Server exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFlexibleServerResult {
        /// The Administrator login for the PostgreSQL Flexible Server.
        pub administrator_login: pulumi_gestalt_rust::Output<String>,
        /// Is the storage auto grow for PostgreSQL Flexible Server enabled?
        pub auto_grow_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The backup retention days for the PostgreSQL Flexible Server.
        pub backup_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the virtual network subnet to create the PostgreSQL Flexible Server.
        pub delegated_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The FQDN of the PostgreSQL Flexible Server.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the PostgreSQL Flexible Server exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Is public network access enabled?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU Name for the PostgreSQL Flexible Server. The name of the SKU, follows the `tier` + `name` pattern (e.g. `B_Standard_B1ms`, `GP_Standard_D2s_v3`, `MO_Standard_E4s_v3`).
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The max storage allowed for the PostgreSQL Flexible Server.
        pub storage_mb: pulumi_gestalt_rust::Output<i32>,
        /// A mapping of tags assigned to the PostgreSQL Flexible Server.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The version of PostgreSQL Flexible Server to use.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFlexibleServerArgs,
    ) -> GetFlexibleServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:postgresql/getFlexibleServer:getFlexibleServer".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFlexibleServerResult {
            administrator_login: o.get_field("administratorLogin"),
            auto_grow_enabled: o.get_field("autoGrowEnabled"),
            backup_retention_days: o.get_field("backupRetentionDays"),
            delegated_subnet_id: o.get_field("delegatedSubnetId"),
            fqdn: o.get_field("fqdn"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            storage_mb: o.get_field("storageMb"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}
