#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedInstanceArgs,
    ) -> GetManagedInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mssql/getManagedInstance:getManagedInstance".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedInstanceResult {
            administrator_login: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("administratorLogin"),
            ),
            collation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("collation"),
            ),
            customer_managed_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerManagedKeyId"),
            ),
            dns_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsZone"),
            ),
            dns_zone_partner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsZonePartnerId"),
            ),
            fqdn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("fqdn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            minimum_tls_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumTlsVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            proxy_override: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("proxyOverride"),
            ),
            public_data_endpoint_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicDataEndpointEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            storage_account_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountType"),
            ),
            storage_size_in_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageSizeInGb"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            timezone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timezoneId"),
            ),
            vcores: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vcores"),
            ),
        }
    }
}
