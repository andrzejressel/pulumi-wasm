#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_flexible_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFlexibleServerArgs {
        /// Specifies the name of the MySQL Flexible Server.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group for the MySQL Flexible Server.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFlexibleServerResult {
        /// The Administrator login of the MySQL Flexible Server.
        pub administrator_login: pulumi_gestalt_rust::Output<String>,
        /// The backup retention days of the MySQL Flexible Server.
        pub backup_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the virtual network subnet the MySQL Flexible Server is created in.
        pub delegated_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified domain name of the MySQL Flexible Server.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Is geo redundant backup enabled?
        pub geo_redundant_backup_enabled: pulumi_gestalt_rust::Output<bool>,
        /// A `high_availability` block for this MySQL Flexible Server as defined below.
        pub high_availabilities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerHighAvailability>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region of the MySQL Flexible Server.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `maintenance_window` block for this MySQL Flexible Server as defined below.
        pub maintenance_windows: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerMaintenanceWindow>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Private DNS zone of the MySQL Flexible Server.
        pub private_dns_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Is the public network access enabled?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The maximum number of replicas that a primary MySQL Flexible Server can have.
        pub replica_capacity: pulumi_gestalt_rust::Output<i32>,
        /// The replication role of the MySQL Flexible Server.
        pub replication_role: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub restore_point_in_time: pulumi_gestalt_rust::Output<String>,
        /// The SKU Name of the MySQL Flexible Server.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A `storage` block for this MySQL Flexible Server as defined below.
        pub storages: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mysql::GetFlexibleServerStorage>,
        >,
        /// A mapping of tags which are assigned to the MySQL Flexible Server.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The version of the MySQL Flexible Server.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zones where this MySQL Flexible Server is located.
        pub zone: pulumi_gestalt_rust::Output<String>,
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
            token: "azure:mysql/getFlexibleServer:getFlexibleServer".into(),
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
            backup_retention_days: o.get_field("backupRetentionDays"),
            delegated_subnet_id: o.get_field("delegatedSubnetId"),
            fqdn: o.get_field("fqdn"),
            geo_redundant_backup_enabled: o.get_field("geoRedundantBackupEnabled"),
            high_availabilities: o.get_field("highAvailabilities"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            maintenance_windows: o.get_field("maintenanceWindows"),
            name: o.get_field("name"),
            private_dns_zone_id: o.get_field("privateDnsZoneId"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            replica_capacity: o.get_field("replicaCapacity"),
            replication_role: o.get_field("replicationRole"),
            resource_group_name: o.get_field("resourceGroupName"),
            restore_point_in_time: o.get_field("restorePointInTime"),
            sku_name: o.get_field("skuName"),
            storages: o.get_field("storages"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
            zone: o.get_field("zone"),
        }
    }
}
