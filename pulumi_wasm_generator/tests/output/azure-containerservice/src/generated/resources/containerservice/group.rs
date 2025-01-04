/// Manages as an Azure Container Group instance.
///
/// ## Example Usage
///
/// This example provisions a Basic Container.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleGroup:
///     type: azure:containerservice:Group
///     name: example
///     properties:
///       name: example-continst
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       ipAddressType: Public
///       dnsNameLabel: aci-label
///       osType: Linux
///       containers:
///         - name: hello-world
///           image: mcr.microsoft.com/azuredocs/aci-helloworld:latest
///           cpu: '0.5'
///           memory: '1.5'
///           ports:
///             - port: 443
///               protocol: TCP
///         - name: sidecar
///           image: mcr.microsoft.com/azuredocs/aci-tutorial-sidecar
///           cpu: '0.5'
///           memory: '1.5'
///       tags:
///         environment: testing
/// ```
///
/// ## Import
///
/// Container Group's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/group:Group containerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerInstance/containerGroups/myContainerGroup1
/// ```
///
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// The definition of a container that is part of the group as documented in the `container` block below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub containers: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::GroupContainer>,
        >,
        /// A `diagnostics` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub diagnostics: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupDiagnostics>,
        >,
        /// A `dns_config` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dns_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupDnsConfig>,
        >,
        /// The DNS label/name for the container group's IP. Changing this forces a new resource to be created.
        ///
        /// > **Note:** DNS label/name is not supported when deploying to virtual networks.
        #[builder(into, default)]
        pub dns_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// The value representing the security enum. `Noreuse`, `ResourceGroupReuse`, `SubscriptionReuse`, `TenantReuse` or `Unsecure`. Defaults to `Unsecure`.
        #[builder(into, default)]
        pub dns_name_label_reuse_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Zero or more `exposed_port` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `exposed_port` can only contain ports that are also exposed on one or more containers in the group.
        #[builder(into, default)]
        pub exposed_ports: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::GroupExposedPort>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupIdentity>,
        >,
        /// An `image_registry_credential` block as documented below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub image_registry_credentials: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerservice::GroupImageRegistryCredential>,
            >,
        >,
        /// The definition of an init container that is part of the group as documented in the `init_container` block below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub init_containers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::GroupInitContainer>>,
        >,
        /// Specifies the IP address type of the container. `Public`, `Private` or `None`. Changing this forces a new resource to be created. If set to `Private`, `subnet_ids` also needs to be set. Defaults to `Public`.
        ///
        /// > **Note:** `dns_name_label` and `os_type` set to `windows` are not compatible with `Private` `ip_address_type`
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Key Vault key URI for CMK encryption. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key_vault_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, the RP principal named "Azure Container Instance Service" will be used instead. Make sure the identity has the proper `key_permissions` set, at least with `Get`, `UnwrapKey`, `WrapKey` and `GetRotationPolicy`.
        #[builder(into, default)]
        pub key_vault_user_assigned_identity_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Container Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub network_profile_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The OS for the container group. Allowed values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** if `os_type` is set to `Windows` currently only a single `container` block is supported. Windows containers are not supported in virtual networks.
        #[builder(into)]
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The priority of the Container Group. Possible values are `Regular` and `Spot`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `priority` is set to `Spot`, the `ip_address_type` has to be `None`.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Container Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Restart policy for the container group. Allowed values are `Always`, `Never`, `OnFailure`. Defaults to `Always`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub restart_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the sku of the Container Group. Possible values are `Confidential`, `Dedicated` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The subnet resource IDs for a container group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub subnet_ids: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this Container Group is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// The definition of a container that is part of the group as documented in the `container` block below. Changing this forces a new resource to be created.
        pub containers: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::GroupContainer>,
        >,
        /// A `diagnostics` block as documented below. Changing this forces a new resource to be created.
        pub diagnostics: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupDiagnostics>,
        >,
        /// A `dns_config` block as documented below. Changing this forces a new resource to be created.
        pub dns_config: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupDnsConfig>,
        >,
        /// The DNS label/name for the container group's IP. Changing this forces a new resource to be created.
        ///
        /// > **Note:** DNS label/name is not supported when deploying to virtual networks.
        pub dns_name_label: pulumi_wasm_rust::Output<Option<String>>,
        /// The value representing the security enum. `Noreuse`, `ResourceGroupReuse`, `SubscriptionReuse`, `TenantReuse` or `Unsecure`. Defaults to `Unsecure`.
        pub dns_name_label_reuse_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Zero or more `exposed_port` blocks as defined below. Changing this forces a new resource to be created.
        ///
        /// > **Note:** The `exposed_port` can only contain ports that are also exposed on one or more containers in the group.
        pub exposed_ports: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::GroupExposedPort>,
        >,
        /// The FQDN of the container group derived from `dns_name_label`.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::GroupIdentity>,
        >,
        /// An `image_registry_credential` block as documented below. Changing this forces a new resource to be created.
        pub image_registry_credentials: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::containerservice::GroupImageRegistryCredential>,
            >,
        >,
        /// The definition of an init container that is part of the group as documented in the `init_container` block below. Changing this forces a new resource to be created.
        pub init_containers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::GroupInitContainer>>,
        >,
        /// The IP address allocated to the container group.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// Specifies the IP address type of the container. `Public`, `Private` or `None`. Changing this forces a new resource to be created. If set to `Private`, `subnet_ids` also needs to be set. Defaults to `Public`.
        ///
        /// > **Note:** `dns_name_label` and `os_type` set to `windows` are not compatible with `Private` `ip_address_type`
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The Key Vault key URI for CMK encryption. Changing this forces a new resource to be created.
        pub key_vault_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The user assigned identity that has access to the Key Vault Key. If not specified, the RP principal named "Azure Container Instance Service" will be used instead. Make sure the identity has the proper `key_permissions` set, at least with `Get`, `UnwrapKey`, `WrapKey` and `GetRotationPolicy`.
        pub key_vault_user_assigned_identity_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Container Group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_profile_id: pulumi_wasm_rust::Output<String>,
        /// The OS for the container group. Allowed values are `Linux` and `Windows`. Changing this forces a new resource to be created.
        ///
        /// > **Note:** if `os_type` is set to `Windows` currently only a single `container` block is supported. Windows containers are not supported in virtual networks.
        pub os_type: pulumi_wasm_rust::Output<String>,
        /// The priority of the Container Group. Possible values are `Regular` and `Spot`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** When `priority` is set to `Spot`, the `ip_address_type` has to be `None`.
        pub priority: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Container Group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Restart policy for the container group. Allowed values are `Always`, `Never`, `OnFailure`. Defaults to `Always`. Changing this forces a new resource to be created.
        pub restart_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the sku of the Container Group. Possible values are `Confidential`, `Dedicated` and `Standard`. Defaults to `Standard`. Changing this forces a new resource to be created.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// The subnet resource IDs for a container group. Changing this forces a new resource to be created.
        pub subnet_ids: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this Container Group is located. Changing this forces a new resource to be created.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupArgs) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let containers_binding = args.containers.get_inner();
        let diagnostics_binding = args.diagnostics.get_inner();
        let dns_config_binding = args.dns_config.get_inner();
        let dns_name_label_binding = args.dns_name_label.get_inner();
        let dns_name_label_reuse_policy_binding = args
            .dns_name_label_reuse_policy
            .get_inner();
        let exposed_ports_binding = args.exposed_ports.get_inner();
        let identity_binding = args.identity.get_inner();
        let image_registry_credentials_binding = args
            .image_registry_credentials
            .get_inner();
        let init_containers_binding = args.init_containers.get_inner();
        let ip_address_type_binding = args.ip_address_type.get_inner();
        let key_vault_key_id_binding = args.key_vault_key_id.get_inner();
        let key_vault_user_assigned_identity_id_binding = args
            .key_vault_user_assigned_identity_id
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_profile_id_binding = args.network_profile_id.get_inner();
        let os_type_binding = args.os_type.get_inner();
        let priority_binding = args.priority.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let restart_policy_binding = args.restart_policy.get_inner();
        let sku_binding = args.sku.get_inner();
        let subnet_ids_binding = args.subnet_ids.get_inner();
        let tags_binding = args.tags.get_inner();
        let zones_binding = args.zones.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/group:Group".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containers".into(),
                    value: &containers_binding,
                },
                register_interface::ObjectField {
                    name: "diagnostics".into(),
                    value: &diagnostics_binding,
                },
                register_interface::ObjectField {
                    name: "dnsConfig".into(),
                    value: &dns_config_binding,
                },
                register_interface::ObjectField {
                    name: "dnsNameLabel".into(),
                    value: &dns_name_label_binding,
                },
                register_interface::ObjectField {
                    name: "dnsNameLabelReusePolicy".into(),
                    value: &dns_name_label_reuse_policy_binding,
                },
                register_interface::ObjectField {
                    name: "exposedPorts".into(),
                    value: &exposed_ports_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "imageRegistryCredentials".into(),
                    value: &image_registry_credentials_binding,
                },
                register_interface::ObjectField {
                    name: "initContainers".into(),
                    value: &init_containers_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultUserAssignedIdentityId".into(),
                    value: &key_vault_user_assigned_identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkProfileId".into(),
                    value: &network_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "osType".into(),
                    value: &os_type_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "restartPolicy".into(),
                    value: &restart_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containers".into(),
                },
                register_interface::ResultField {
                    name: "diagnostics".into(),
                },
                register_interface::ResultField {
                    name: "dnsConfig".into(),
                },
                register_interface::ResultField {
                    name: "dnsNameLabel".into(),
                },
                register_interface::ResultField {
                    name: "dnsNameLabelReusePolicy".into(),
                },
                register_interface::ResultField {
                    name: "exposedPorts".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "imageRegistryCredentials".into(),
                },
                register_interface::ResultField {
                    name: "initContainers".into(),
                },
                register_interface::ResultField {
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultKeyId".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultUserAssignedIdentityId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfileId".into(),
                },
                register_interface::ResultField {
                    name: "osType".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "restartPolicy".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zones".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupResult {
            containers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containers").unwrap(),
            ),
            diagnostics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diagnostics").unwrap(),
            ),
            dns_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsConfig").unwrap(),
            ),
            dns_name_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsNameLabel").unwrap(),
            ),
            dns_name_label_reuse_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsNameLabelReusePolicy").unwrap(),
            ),
            exposed_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exposedPorts").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            image_registry_credentials: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageRegistryCredentials").unwrap(),
            ),
            init_containers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initContainers").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            key_vault_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultKeyId").unwrap(),
            ),
            key_vault_user_assigned_identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultUserAssignedIdentityId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfileId").unwrap(),
            ),
            os_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("osType").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            restart_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restartPolicy").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zones").unwrap(),
            ),
        }
    }
}
