/// Manages a Volume Quota Rule.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("example-netappaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePool = pool::create(
///         "examplePool",
///         PoolArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .location("${example.location}")
///             .name("example-netapppool")
///             .resource_group_name("${example.name}")
///             .service_level("Premium")
///             .size_in_tb(4)
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("netapp")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/networkinterfaces/*",
///                     "Microsoft.Network/virtualNetworks/subnets/join/action",])
///                     .name("Microsoft.Netapp/volumes").build_struct()).build_struct(),
///                 ],
///             )
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-virtualnetwork")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVolume = volume::create(
///         "exampleVolume",
///         VolumeArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .location("${example.location}")
///             .name("example-netappvolume")
///             .network_features("Basic")
///             .pool_name("${examplePool.name}")
///             .protocols(vec!["NFSv4.1",])
///             .resource_group_name("${example.name}")
///             .security_style("unix")
///             .service_level("Premium")
///             .snapshot_directory_visible(false)
///             .storage_quota_in_gb(100)
///             .subnet_id("${exampleSubnet.id}")
///             .volume_path("my-unique-file-path")
///             .zone("1")
///             .build_struct(),
///     );
///     let quota1 = volume_quota_rule::create(
///         "quota1",
///         VolumeQuotaRuleArgs::builder()
///             .location("${example.location}")
///             .name("example-quota-rule-1")
///             .quota_size_in_kib(1024)
///             .quota_target("3001")
///             .quota_type("IndividualGroupQuota")
///             .volume_id("${exampleVolume.id}")
///             .build_struct(),
///     );
///     let quota2 = volume_quota_rule::create(
///         "quota2",
///         VolumeQuotaRuleArgs::builder()
///             .location("${example.location}")
///             .name("example-quota-rule-2")
///             .quota_size_in_kib(1024)
///             .quota_target("2001")
///             .quota_type("IndividualUserQuota")
///             .volume_id("${exampleVolume.id}")
///             .build_struct(),
///     );
///     let quota3 = volume_quota_rule::create(
///         "quota3",
///         VolumeQuotaRuleArgs::builder()
///             .location("${example.location}")
///             .name("example-quota-rule-3")
///             .quota_size_in_kib(1024)
///             .quota_type("DefaultUserQuota")
///             .volume_id("${exampleVolume.id}")
///             .build_struct(),
///     );
///     let quota4 = volume_quota_rule::create(
///         "quota4",
///         VolumeQuotaRuleArgs::builder()
///             .location("${example.location}")
///             .name("example-quota-rule-4")
///             .quota_size_in_kib(1024)
///             .quota_type("DefaultGroupQuota")
///             .volume_id("${exampleVolume.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Volume Quota Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/volumeQuotaRule:VolumeQuotaRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.NetApp/netAppAccounts/account1/capacityPools/pool1/volumes/vol1/volumeQuotaRules/quota1
/// ```
///
pub mod volume_quota_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeQuotaRuleArgs {
        /// The Azure Region where the Volume Quota Rule should exist. Changing this forces a new Volume Quota Rule to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Volume Quota Rule. Changing this forces a new Volume Quota Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota size in kibibytes.
        #[builder(into)]
        pub quota_size_in_kib: pulumi_wasm_rust::Output<i32>,
        /// Quota Target. This can be Unix UID/GID for NFSv3/NFSv4.1 volumes and Windows User SID for CIFS based volumes. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `quota_target ` must be used when `quota_type` is `IndividualGroupQuota` or `IndividualUserQuota`
        ///
        /// > **NOTE:** more information about this resource can be found at [Understand default and individual user and group quotas](https://learn.microsoft.com/en-us/azure/azure-netapp-files/default-individual-user-group-quotas-introduction)
        #[builder(into, default)]
        pub quota_target: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota type. Possible values are `DefaultGroupQuota`, `DefaultUserQuota`, `IndividualGroupQuota` and `IndividualUserQuota`. Please note that `IndividualGroupQuota` and `DefaultGroupQuota` are not applicable to SMB and dual-protocol volumes. Changing this forces a new resource to be created.
        #[builder(into)]
        pub quota_type: pulumi_wasm_rust::Output<String>,
        /// The NetApp volume ID where the Volume Quota Rule is assigned to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VolumeQuotaRuleResult {
        /// The Azure Region where the Volume Quota Rule should exist. Changing this forces a new Volume Quota Rule to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Volume Quota Rule. Changing this forces a new Volume Quota Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Quota size in kibibytes.
        pub quota_size_in_kib: pulumi_wasm_rust::Output<i32>,
        /// Quota Target. This can be Unix UID/GID for NFSv3/NFSv4.1 volumes and Windows User SID for CIFS based volumes. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `quota_target ` must be used when `quota_type` is `IndividualGroupQuota` or `IndividualUserQuota`
        ///
        /// > **NOTE:** more information about this resource can be found at [Understand default and individual user and group quotas](https://learn.microsoft.com/en-us/azure/azure-netapp-files/default-individual-user-group-quotas-introduction)
        pub quota_target: pulumi_wasm_rust::Output<Option<String>>,
        /// Quota type. Possible values are `DefaultGroupQuota`, `DefaultUserQuota`, `IndividualGroupQuota` and `IndividualUserQuota`. Please note that `IndividualGroupQuota` and `DefaultGroupQuota` are not applicable to SMB and dual-protocol volumes. Changing this forces a new resource to be created.
        pub quota_type: pulumi_wasm_rust::Output<String>,
        /// The NetApp volume ID where the Volume Quota Rule is assigned to. Changing this forces a new resource to be created.
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VolumeQuotaRuleArgs) -> VolumeQuotaRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let quota_size_in_kib_binding = args.quota_size_in_kib.get_inner();
        let quota_target_binding = args.quota_target.get_inner();
        let quota_type_binding = args.quota_type.get_inner();
        let volume_id_binding = args.volume_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/volumeQuotaRule:VolumeQuotaRule".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "quotaSizeInKib".into(),
                    value: &quota_size_in_kib_binding,
                },
                register_interface::ObjectField {
                    name: "quotaTarget".into(),
                    value: &quota_target_binding,
                },
                register_interface::ObjectField {
                    name: "quotaType".into(),
                    value: &quota_type_binding,
                },
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "quotaSizeInKib".into(),
                },
                register_interface::ResultField {
                    name: "quotaTarget".into(),
                },
                register_interface::ResultField {
                    name: "quotaType".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VolumeQuotaRuleResult {
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            quota_size_in_kib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaSizeInKib").unwrap(),
            ),
            quota_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaTarget").unwrap(),
            ),
            quota_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaType").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
        }
    }
}
