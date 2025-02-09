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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_sim_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimGroupArgs {
        /// A key to encrypt the SIM data that belongs to this SIM group.
        #[builder(into, default)]
        pub encryption_key_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        ///
        /// > **NOTE:** A `UserAssigned` identity must be specified when `encryption_key_url` is specified.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::mobile::NetworkSimGroupIdentity>,
        >,
        /// Specifies the Azure Region where the Mobile Network Sim Groups should exist. Changing this forces a new Mobile Network Sim Group to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of Mobile Network which the Mobile Network Sim Group belongs to. Changing this forces a new Mobile Network Slice to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name which should be used for this Mobile Network Sim Groups. Changing this forces a new Mobile Network Sim Group to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Groups.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimGroupResult {
        /// A key to encrypt the SIM data that belongs to this SIM group.
        pub encryption_key_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        ///
        /// > **NOTE:** A `UserAssigned` identity must be specified when `encryption_key_url` is specified.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::mobile::NetworkSimGroupIdentity>,
        >,
        /// Specifies the Azure Region where the Mobile Network Sim Groups should exist. Changing this forces a new Mobile Network Sim Group to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of Mobile Network which the Mobile Network Sim Group belongs to. Changing this forces a new Mobile Network Slice to be created.
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Mobile Network Sim Groups. Changing this forces a new Mobile Network Sim Group to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Groups.
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
        args: NetworkSimGroupArgs,
    ) -> NetworkSimGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let encryption_key_url_binding_1 = args.encryption_key_url.get_output(context);
        let encryption_key_url_binding = encryption_key_url_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let mobile_network_id_binding_1 = args.mobile_network_id.get_output(context);
        let mobile_network_id_binding = mobile_network_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkSimGroup:NetworkSimGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NetworkSimGroupResult {
            encryption_key_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionKeyUrl"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mobile_network_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mobileNetworkId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
