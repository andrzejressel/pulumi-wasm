#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedInstanceArgs {
        /// The name of the SQL Managed Instance.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group where the SQL Managed Instance exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedInstanceResult {
        /// The administrator login name for the SQL Managed Instance.
        pub administrator_login: pulumi_gestalt_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be collated.
        pub collation: pulumi_gestalt_rust::Output<String>,
        pub customer_managed_key_id: pulumi_gestalt_rust::Output<String>,
        /// The Dns Zone where the SQL Managed Instance is located.
        pub dns_zone: pulumi_gestalt_rust::Output<String>,
        /// The ID of the SQL Managed Instance which shares the DNS zone.
        pub dns_zone_partner_id: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified domain name of the Azure Managed SQL Instance.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mssql::GetManagedInstanceIdentity>,
        >,
        /// What type of license the SQL Managed Instance uses.
        pub license_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Minimum TLS Version.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be accessed.
        pub proxy_override: pulumi_gestalt_rust::Output<String>,
        /// Whether the public data endpoint is enabled.
        pub public_data_endpoint_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU Name of the SQL Managed Instance.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the storage account type used to store backups for this database.
        pub storage_account_type: pulumi_gestalt_rust::Output<String>,
        /// Maximum storage space allocated for the SQL Managed Instance.
        pub storage_size_in_gb: pulumi_gestalt_rust::Output<i32>,
        /// The subnet resource ID that the SQL Managed Instance is associated with.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The TimeZone ID that the SQL Managed Instance is running in.
        pub timezone_id: pulumi_gestalt_rust::Output<String>,
        /// Number of cores that are assigned to the SQL Managed Instance.
        pub vcores: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedInstanceArgs,
    ) -> GetManagedInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mssql/getManagedInstance:getManagedInstance".into(),
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
        GetManagedInstanceResult {
            administrator_login: o.get_field("administratorLogin"),
            collation: o.get_field("collation"),
            customer_managed_key_id: o.get_field("customerManagedKeyId"),
            dns_zone: o.get_field("dnsZone"),
            dns_zone_partner_id: o.get_field("dnsZonePartnerId"),
            fqdn: o.get_field("fqdn"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            license_type: o.get_field("licenseType"),
            location: o.get_field("location"),
            minimum_tls_version: o.get_field("minimumTlsVersion"),
            name: o.get_field("name"),
            proxy_override: o.get_field("proxyOverride"),
            public_data_endpoint_enabled: o.get_field("publicDataEndpointEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            storage_account_type: o.get_field("storageAccountType"),
            storage_size_in_gb: o.get_field("storageSizeInGb"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            timezone_id: o.get_field("timezoneId"),
            vcores: o.get_field("vcores"),
        }
    }
}
