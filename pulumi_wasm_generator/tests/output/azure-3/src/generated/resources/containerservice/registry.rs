/// Manages an Azure Container Registry.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   acr:
///     type: azure:containerservice:Registry
///     properties:
///       name: containerRegistry1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Premium
///       adminEnabled: false
///       georeplications:
///         - location: East US
///           zoneRedundancyEnabled: true
///           tags: {}
///         - location: North Europe
///           zoneRedundancyEnabled: true
///           tags: {}
/// ```
///
///
/// ### Encryption)
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   acr:
///     type: azure:containerservice:Registry
///     properties:
///       name: containerRegistry1
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       sku: Premium
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       encryption:
///         keyVaultKeyId: ${example.id}
///         identityClientId: ${exampleUserAssignedIdentity.clientId}
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       name: registry-uai
/// variables:
///   example:
///     fn::invoke:
///       function: azure:keyvault:getKey
///       arguments:
///         name: super-secret
///         keyVaultId: ${existing.id}
/// ```
///
///
/// ### Attaching A Container Registry To A Kubernetes Cluster)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleRegistry:
///     type: azure:containerservice:Registry
///     name: example
///     properties:
///       name: containerRegistry1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Premium
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example-aks1
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: exampleaks1
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_D2_v2
///       identity:
///         type: SystemAssigned
///       tags:
///         Environment: Production
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       principalId: ${exampleKubernetesCluster.kubeletIdentity.objectId}
///       roleDefinitionName: AcrPull
///       scope: ${exampleRegistry.id}
///       skipServicePrincipalAadCheck: true
/// ```
///
/// ## Import
///
/// Container Registries can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/registry:Registry example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ContainerRegistry/registries/myregistry1
/// ```
///
pub mod registry {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryArgs {
        /// Specifies whether the admin user is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub admin_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether allows anonymous (unauthenticated) pull access to this Container Registry? This is only supported on resources with the `Standard` or `Premium` SKU.
        #[builder(into, default)]
        pub anonymous_pull_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether to enable dedicated data endpoints for this Container Registry? This is only supported on resources with the `Premium` SKU.
        #[builder(into, default)]
        pub data_endpoint_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An `encryption` block as documented below.
        #[builder(into, default)]
        pub encryption: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryEncryption>,
        >,
        /// Boolean value that indicates whether export policy is enabled. Defaults to `true`. In order to set it to `false`, make sure the `public_network_access_enabled` is also set to `false`.
        ///
        /// > **NOTE:** `quarantine_policy_enabled`, `retention_policy_in_days`, `trust_policy_enabled`, `export_policy_enabled` and `zone_redundancy_enabled` are only supported on resources with the `Premium` SKU.
        #[builder(into, default)]
        pub export_policy_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// One or more `georeplications` blocks as documented below.
        ///
        /// > **NOTE:** The `georeplications` is only supported on new resources with the `Premium` SKU.
        ///
        /// > **NOTE:** The `georeplications` list cannot contain the location where the Container Registry exists.
        ///
        /// > **NOTE:** If more than one `georeplications` block is specified, they are expected to follow the alphabetic order on the `location` property.
        #[builder(into, default)]
        pub georeplications: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::containerservice::RegistryGeoreplication>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Container Registry. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `AzureServices`.
        #[builder(into, default)]
        pub network_rule_bypass_option: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `network_rule_set` block as documented below.
        #[builder(into, default)]
        pub network_rule_set: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::containerservice::RegistryNetworkRuleSet>,
        >,
        /// Whether public network access is allowed for the container registry. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Boolean value that indicates whether quarantine policy is enabled.
        #[builder(into, default)]
        pub quarantine_policy_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group in which to create the Container Registry. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of days to retain and untagged manifest after which it gets purged. Defaults to `7`.
        #[builder(into, default)]
        pub retention_policy_in_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The SKU name of the container registry. Possible values are `Basic`, `Standard` and `Premium`.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean value that indicated whether trust policy is enabled. Defaults to `false`.
        #[builder(into, default)]
        pub trust_policy_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Whether zone redundancy is enabled for this Container Registry? Changing this forces a new resource to be created. Defaults to `false`.
        #[builder(into, default)]
        pub zone_redundancy_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RegistryResult {
        /// Specifies whether the admin user is enabled. Defaults to `false`.
        pub admin_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Password associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_password: pulumi_wasm_rust::Output<String>,
        /// The Username associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_username: pulumi_wasm_rust::Output<String>,
        /// Whether allows anonymous (unauthenticated) pull access to this Container Registry? This is only supported on resources with the `Standard` or `Premium` SKU.
        pub anonymous_pull_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether to enable dedicated data endpoints for this Container Registry? This is only supported on resources with the `Premium` SKU.
        pub data_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `encryption` block as documented below.
        pub encryption: pulumi_wasm_rust::Output<
            super::super::types::containerservice::RegistryEncryption,
        >,
        /// Boolean value that indicates whether export policy is enabled. Defaults to `true`. In order to set it to `false`, make sure the `public_network_access_enabled` is also set to `false`.
        ///
        /// > **NOTE:** `quarantine_policy_enabled`, `retention_policy_in_days`, `trust_policy_enabled`, `export_policy_enabled` and `zone_redundancy_enabled` are only supported on resources with the `Premium` SKU.
        pub export_policy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// One or more `georeplications` blocks as documented below.
        ///
        /// > **NOTE:** The `georeplications` is only supported on new resources with the `Premium` SKU.
        ///
        /// > **NOTE:** The `georeplications` list cannot contain the location where the Container Registry exists.
        ///
        /// > **NOTE:** If more than one `georeplications` block is specified, they are expected to follow the alphabetic order on the `location` property.
        pub georeplications: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::RegistryGeoreplication>>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::containerservice::RegistryIdentity>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The URL that can be used to log into the container registry.
        pub login_server: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Container Registry. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether to allow trusted Azure services to access a network restricted Container Registry? Possible values are `None` and `AzureServices`. Defaults to `AzureServices`.
        pub network_rule_bypass_option: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_rule_set` block as documented below.
        pub network_rule_set: pulumi_wasm_rust::Output<
            super::super::types::containerservice::RegistryNetworkRuleSet,
        >,
        /// Whether public network access is allowed for the container registry. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Boolean value that indicates whether quarantine policy is enabled.
        pub quarantine_policy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the Container Registry. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The number of days to retain and untagged manifest after which it gets purged. Defaults to `7`.
        pub retention_policy_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The SKU name of the container registry. Possible values are `Basic`, `Standard` and `Premium`.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Boolean value that indicated whether trust policy is enabled. Defaults to `false`.
        pub trust_policy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether zone redundancy is enabled for this Container Registry? Changing this forces a new resource to be created. Defaults to `false`.
        pub zone_redundancy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegistryArgs,
    ) -> RegistryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admin_enabled_binding = args.admin_enabled.get_output(context).get_inner();
        let anonymous_pull_enabled_binding = args
            .anonymous_pull_enabled
            .get_output(context)
            .get_inner();
        let data_endpoint_enabled_binding = args
            .data_endpoint_enabled
            .get_output(context)
            .get_inner();
        let encryption_binding = args.encryption.get_output(context).get_inner();
        let export_policy_enabled_binding = args
            .export_policy_enabled
            .get_output(context)
            .get_inner();
        let georeplications_binding = args
            .georeplications
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_rule_bypass_option_binding = args
            .network_rule_bypass_option
            .get_output(context)
            .get_inner();
        let network_rule_set_binding = args
            .network_rule_set
            .get_output(context)
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let quarantine_policy_enabled_binding = args
            .quarantine_policy_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let retention_policy_in_days_binding = args
            .retention_policy_in_days
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let trust_policy_enabled_binding = args
            .trust_policy_enabled
            .get_output(context)
            .get_inner();
        let zone_redundancy_enabled_binding = args
            .zone_redundancy_enabled
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/registry:Registry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "adminEnabled".into(),
                    value: &admin_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "anonymousPullEnabled".into(),
                    value: &anonymous_pull_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "dataEndpointEnabled".into(),
                    value: &data_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "encryption".into(),
                    value: &encryption_binding,
                },
                register_interface::ObjectField {
                    name: "exportPolicyEnabled".into(),
                    value: &export_policy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "georeplications".into(),
                    value: &georeplications_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
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
                    name: "networkRuleBypassOption".into(),
                    value: &network_rule_bypass_option_binding,
                },
                register_interface::ObjectField {
                    name: "networkRuleSet".into(),
                    value: &network_rule_set_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "quarantinePolicyEnabled".into(),
                    value: &quarantine_policy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionPolicyInDays".into(),
                    value: &retention_policy_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trustPolicyEnabled".into(),
                    value: &trust_policy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundancyEnabled".into(),
                    value: &zone_redundancy_enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegistryResult {
            admin_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminEnabled"),
            ),
            admin_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminPassword"),
            ),
            admin_username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("adminUsername"),
            ),
            anonymous_pull_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("anonymousPullEnabled"),
            ),
            data_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataEndpointEnabled"),
            ),
            encryption: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryption"),
            ),
            export_policy_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("exportPolicyEnabled"),
            ),
            georeplications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("georeplications"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            login_server: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loginServer"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network_rule_bypass_option: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkRuleBypassOption"),
            ),
            network_rule_set: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networkRuleSet"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            quarantine_policy_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("quarantinePolicyEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            retention_policy_in_days: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retentionPolicyInDays"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            trust_policy_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("trustPolicyEnabled"),
            ),
            zone_redundancy_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("zoneRedundancyEnabled"),
            ),
        }
    }
}
