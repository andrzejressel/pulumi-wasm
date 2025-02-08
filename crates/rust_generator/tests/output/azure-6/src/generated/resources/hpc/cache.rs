/// Manages a HPC Cache.
///
/// > **Note:** By request of the service team the provider no longer automatically registering the `Microsoft.StorageCache` Resource Provider for this resource. To register it you can run `az provider register --namespace 'Microsoft.StorageCache'`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleCache = cache::create(
///         "exampleCache",
///         CacheArgs::builder()
///             .cache_size_in_gb(3072)
///             .location("${example.location}")
///             .name("examplehpccache")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard_2G")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .name("examplesubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("examplevn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HPC Caches can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hpc/cache:Cache example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroupName/providers/Microsoft.StorageCache/caches/cacheName
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cache {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CacheArgs {
        /// Specifies whether the HPC Cache automatically rotates Encryption Key to the latest version.
        #[builder(into, default)]
        pub automatically_rotate_key_to_latest_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The size of the HPC Cache, in GB. Possible values are `3072`, `6144`, `12288`, `21623`, `24576`, `43246`, `49152` and `86491`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `21623`, `43246` and `86491` sizes are restricted to read only resources.
        #[builder(into)]
        pub cache_size_in_gb: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A `default_access_policy` block as defined below.
        #[builder(into, default)]
        pub default_access_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheDefaultAccessPolicy>,
        >,
        /// A `directory_active_directory` block as defined below.
        #[builder(into, default)]
        pub directory_active_directory: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheDirectoryActiveDirectory>,
        >,
        /// A `directory_flat_file` block as defined below.
        #[builder(into, default)]
        pub directory_flat_file: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheDirectoryFlatFile>,
        >,
        /// A `directory_ldap` block as defined below.
        ///
        /// > **Note:** Only one of `directory_active_directory`, `directory_flat_file` and `directory_ldap` can be set.
        #[builder(into, default)]
        pub directory_ldap: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheDirectoryLdap>,
        >,
        /// A `dns` block as defined below.
        #[builder(into, default)]
        pub dns: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheDns>,
        >,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hpc::CacheIdentity>,
        >,
        /// The ID of the Key Vault Key which should be used to encrypt the data in this HPC Cache.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the supported Azure Region where the HPC Cache should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 maximum transmission unit configured for the subnet of the HPC Cache. Possible values range from 576 - 1500. Defaults to `1500`.
        #[builder(into, default)]
        pub mtu: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the HPC Cache. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The NTP server IP Address or FQDN for the HPC Cache. Defaults to `time.windows.com`.
        #[builder(into, default)]
        pub ntp_server: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which to create the HPC Cache. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of HPC Cache to use. Possible values are (ReadWrite) - `Standard_2G`, `Standard_4G` `Standard_8G` or (ReadOnly) - `Standard_L4_5G`, `Standard_L9G`, and `Standard_L16G`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The read-only SKUs have restricted cache sizes. `Standard_L4_5G` must be set to `21623`. `Standard_L9G` to `43246` and `Standard_L16G` to `86491`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Subnet for the HPC Cache. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the HPC Cache.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CacheResult {
        /// Specifies whether the HPC Cache automatically rotates Encryption Key to the latest version.
        pub automatically_rotate_key_to_latest_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The size of the HPC Cache, in GB. Possible values are `3072`, `6144`, `12288`, `21623`, `24576`, `43246`, `49152` and `86491`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `21623`, `43246` and `86491` sizes are restricted to read only resources.
        pub cache_size_in_gb: pulumi_gestalt_rust::Output<i32>,
        /// A `default_access_policy` block as defined below.
        pub default_access_policy: pulumi_gestalt_rust::Output<
            super::super::types::hpc::CacheDefaultAccessPolicy,
        >,
        /// A `directory_active_directory` block as defined below.
        pub directory_active_directory: pulumi_gestalt_rust::Output<
            Option<super::super::types::hpc::CacheDirectoryActiveDirectory>,
        >,
        /// A `directory_flat_file` block as defined below.
        pub directory_flat_file: pulumi_gestalt_rust::Output<
            Option<super::super::types::hpc::CacheDirectoryFlatFile>,
        >,
        /// A `directory_ldap` block as defined below.
        ///
        /// > **Note:** Only one of `directory_active_directory`, `directory_flat_file` and `directory_ldap` can be set.
        pub directory_ldap: pulumi_gestalt_rust::Output<
            Option<super::super::types::hpc::CacheDirectoryLdap>,
        >,
        /// A `dns` block as defined below.
        pub dns: pulumi_gestalt_rust::Output<Option<super::super::types::hpc::CacheDns>>,
        /// An `identity` block as defined below. Changing this forces a new resource to be created.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::hpc::CacheIdentity>,
        >,
        /// The ID of the Key Vault Key which should be used to encrypt the data in this HPC Cache.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the supported Azure Region where the HPC Cache should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A list of IP Addresses where the HPC Cache can be mounted.
        pub mount_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IPv4 maximum transmission unit configured for the subnet of the HPC Cache. Possible values range from 576 - 1500. Defaults to `1500`.
        pub mtu: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the HPC Cache. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The NTP server IP Address or FQDN for the HPC Cache. Defaults to `time.windows.com`.
        pub ntp_server: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which to create the HPC Cache. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of HPC Cache to use. Possible values are (ReadWrite) - `Standard_2G`, `Standard_4G` `Standard_8G` or (ReadOnly) - `Standard_L4_5G`, `Standard_L9G`, and `Standard_L16G`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The read-only SKUs have restricted cache sizes. `Standard_L4_5G` must be set to `21623`. `Standard_L9G` to `43246` and `Standard_L16G` to `86491`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Subnet for the HPC Cache. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the HPC Cache.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CacheArgs,
    ) -> CacheResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automatically_rotate_key_to_latest_enabled_binding = args
            .automatically_rotate_key_to_latest_enabled
            .get_output(context)
            .get_inner();
        let cache_size_in_gb_binding = args
            .cache_size_in_gb
            .get_output(context)
            .get_inner();
        let default_access_policy_binding = args
            .default_access_policy
            .get_output(context)
            .get_inner();
        let directory_active_directory_binding = args
            .directory_active_directory
            .get_output(context)
            .get_inner();
        let directory_flat_file_binding = args
            .directory_flat_file
            .get_output(context)
            .get_inner();
        let directory_ldap_binding = args.directory_ldap.get_output(context).get_inner();
        let dns_binding = args.dns.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mtu_binding = args.mtu.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let ntp_server_binding = args.ntp_server.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hpc/cache:Cache".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticallyRotateKeyToLatestEnabled".into(),
                    value: &automatically_rotate_key_to_latest_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "cacheSizeInGb".into(),
                    value: &cache_size_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAccessPolicy".into(),
                    value: &default_access_policy_binding,
                },
                register_interface::ObjectField {
                    name: "directoryActiveDirectory".into(),
                    value: &directory_active_directory_binding,
                },
                register_interface::ObjectField {
                    name: "directoryFlatFile".into(),
                    value: &directory_flat_file_binding,
                },
                register_interface::ObjectField {
                    name: "directoryLdap".into(),
                    value: &directory_ldap_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mtu".into(),
                    value: &mtu_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ntpServer".into(),
                    value: &ntp_server_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CacheResult {
            automatically_rotate_key_to_latest_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automaticallyRotateKeyToLatestEnabled"),
            ),
            cache_size_in_gb: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheSizeInGb"),
            ),
            default_access_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultAccessPolicy"),
            ),
            directory_active_directory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryActiveDirectory"),
            ),
            directory_flat_file: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryFlatFile"),
            ),
            directory_ldap: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryLdap"),
            ),
            dns: pulumi_gestalt_rust::__private::into_domain(o.extract_field("dns")),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mount_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mountAddresses"),
            ),
            mtu: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mtu")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            ntp_server: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ntpServer"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
