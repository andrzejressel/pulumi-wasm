/// Manages a Mobile Network Sim Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkSimGroup:
///     type: azure:mobile:NetworkSimGroup
///     name: example
///     properties:
///       name: example-mnsg
///       location: ${exampleResourceGroup.location}
///       mobileNetworkId: ${exampleNetwork.id}
///       encryptionKeyUrl: ${exampleGetKey.id}
///       identity:
///         type: SystemAssigned, UserAssigned
///         identityIds:
///           - ${example.id}
///       tags:
///         key: value
/// variables:
///   example:
///     fn::invoke:
///       function: azure:authorization:getUserAssignedIdentity
///       arguments:
///         name: name_of_user_assigned_identity
///         resourceGroupName: name_of_resource_group
///   exampleGetKeyVault:
///     fn::invoke:
///       function: azure:keyvault:getKeyVault
///       arguments:
///         name: example-kv
///         resourceGroupName: some-resource-group
///   exampleGetKey:
///     fn::invoke:
///       function: azure:keyvault:getKey
///       arguments:
///         name: example-key
///         keyVaultId: ${exampleGetKeyVault.id}
/// ```
///
/// ## Import
///
/// Mobile Network Sim Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSimGroup:NetworkSimGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/simGroups/simGroup1
/// ```
///
pub mod network_sim_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimGroupArgs {
        /// A key to encrypt the SIM data that belongs to this SIM group.
        #[builder(into, default)]
        pub encryption_key_url: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        ///
        /// > **NOTE:** A `UserAssigned` identity must be specified when `encryption_key_url` is specified.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mobile::NetworkSimGroupIdentity>,
        >,
        /// Specifies the Azure Region where the Mobile Network Sim Groups should exist. Changing this forces a new Mobile Network Sim Group to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of Mobile Network which the Mobile Network Sim Group belongs to. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Sim Groups. Changing this forces a new Mobile Network Sim Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Groups.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimGroupResult {
        /// A key to encrypt the SIM data that belongs to this SIM group.
        pub encryption_key_url: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        ///
        /// > **NOTE:** A `UserAssigned` identity must be specified when `encryption_key_url` is specified.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mobile::NetworkSimGroupIdentity>,
        >,
        /// Specifies the Azure Region where the Mobile Network Sim Groups should exist. Changing this forces a new Mobile Network Sim Group to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of Mobile Network which the Mobile Network Sim Group belongs to. Changing this forces a new Mobile Network Slice to be created.
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Sim Groups. Changing this forces a new Mobile Network Sim Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Groups.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkSimGroupArgs) -> NetworkSimGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let encryption_key_url_binding = args.encryption_key_url.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkSimGroup:NetworkSimGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionKeyUrl".into(),
                    value: &encryption_key_url_binding,
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
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "encryptionKeyUrl".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkSimGroupResult {
            encryption_key_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionKeyUrl").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
