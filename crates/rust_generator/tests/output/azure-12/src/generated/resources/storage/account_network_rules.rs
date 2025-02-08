/// Manages network rules inside of a Azure Storage Account.
///
/// > **NOTE:** Network Rules can be defined either directly on the `azure.storage.Account` resource, or using the `azure.storage.AccountNetworkRules` resource - but the two cannot be used together. Spurious changes will occur if both are used against the same Storage Account.
///
/// > **NOTE:** Only one `azure.storage.AccountNetworkRules` can be tied to an `azure.storage.Account`. Spurious changes will occur if more than `azure.storage.AccountNetworkRules` is tied to the same `azure.storage.Account`.
///
/// > **NOTE:** Deleting this resource updates the storage account back to the default values it had when the storage account was created.
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       serviceEndpoints:
///         - Microsoft.Storage
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: storageaccountname
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: GRS
///       tags:
///         environment: staging
///   exampleAccountNetworkRules:
///     type: azure:storage:AccountNetworkRules
///     name: example
///     properties:
///       storageAccountId: ${exampleAccount.id}
///       defaultAction: Allow
///       ipRules:
///         - 127.0.0.1
///       virtualNetworkSubnetIds:
///         - ${exampleSubnet.id}
///       bypasses:
///         - Metrics
/// ```
///
/// ## Import
///
/// Storage Account Network Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/accountNetworkRules:AccountNetworkRules storageAcc1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Storage/storageAccounts/myaccount
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod account_network_rules {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountNetworkRulesArgs {
        /// Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Valid options are any combination of `Logging`, `Metrics`, `AzureServices`, or `None`. Defaults to `["AzureServices"]`.
        ///
        /// > **NOTE** User has to explicitly set `bypass` to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub bypasses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the default action of allow or deny when no other rules match. Valid options are `Deny` or `Allow`.
        #[builder(into)]
        pub default_action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of public IP or IP ranges in CIDR Format. Only IPv4 addresses are allowed. Private IP address ranges (as defined in [RFC 1918](https://tools.ietf.org/html/rfc1918#section-3)) are not allowed.
        ///
        /// > **NOTE** Small address ranges using "/31" or "/32" prefix sizes are not supported. These ranges should be configured using individual IP address rules without prefix specified.
        ///
        /// > **NOTE** IP network rules have no effect on requests originating from the same Azure region as the storage account. Use Virtual network rules to allow same-region requests. Services deployed in the same region as the storage account use private Azure IP addresses for communication. Thus, you cannot restrict access to specific Azure services based on their public outbound IP address range.
        ///
        /// > **NOTE** User has to explicitly set `ip_rules` to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub ip_rules: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `private_link_access` block as defined below.
        #[builder(into, default)]
        pub private_link_access_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::storage::AccountNetworkRulesPrivateLinkAccessRule,
                >,
            >,
        >,
        /// Specifies the ID of the storage account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of virtual network subnet ids to secure the storage account.
        ///
        /// > **NOTE** User has to explicitly set `virtual_network_subnet_ids` to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub virtual_network_subnet_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountNetworkRulesResult {
        /// Specifies whether traffic is bypassed for Logging/Metrics/AzureServices. Valid options are any combination of `Logging`, `Metrics`, `AzureServices`, or `None`. Defaults to `["AzureServices"]`.
        ///
        /// > **NOTE** User has to explicitly set `bypass` to empty slice (`[]`) to remove it.
        pub bypasses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies the default action of allow or deny when no other rules match. Valid options are `Deny` or `Allow`.
        pub default_action: pulumi_gestalt_rust::Output<String>,
        /// List of public IP or IP ranges in CIDR Format. Only IPv4 addresses are allowed. Private IP address ranges (as defined in [RFC 1918](https://tools.ietf.org/html/rfc1918#section-3)) are not allowed.
        ///
        /// > **NOTE** Small address ranges using "/31" or "/32" prefix sizes are not supported. These ranges should be configured using individual IP address rules without prefix specified.
        ///
        /// > **NOTE** IP network rules have no effect on requests originating from the same Azure region as the storage account. Use Virtual network rules to allow same-region requests. Services deployed in the same region as the storage account use private Azure IP addresses for communication. Thus, you cannot restrict access to specific Azure services based on their public outbound IP address range.
        ///
        /// > **NOTE** User has to explicitly set `ip_rules` to empty slice (`[]`) to remove it.
        pub ip_rules: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `private_link_access` block as defined below.
        pub private_link_access_rules: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::storage::AccountNetworkRulesPrivateLinkAccessRule,
                >,
            >,
        >,
        /// Specifies the ID of the storage account. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of virtual network subnet ids to secure the storage account.
        ///
        /// > **NOTE** User has to explicitly set `virtual_network_subnet_ids` to empty slice (`[]`) to remove it.
        pub virtual_network_subnet_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AccountNetworkRulesArgs,
    ) -> AccountNetworkRulesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bypasses_binding = args.bypasses.get_output(context).get_inner();
        let default_action_binding = args.default_action.get_output(context).get_inner();
        let ip_rules_binding = args.ip_rules.get_output(context).get_inner();
        let private_link_access_rules_binding = args
            .private_link_access_rules
            .get_output(context)
            .get_inner();
        let storage_account_id_binding = args
            .storage_account_id
            .get_output(context)
            .get_inner();
        let virtual_network_subnet_ids_binding = args
            .virtual_network_subnet_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/accountNetworkRules:AccountNetworkRules".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bypasses".into(),
                    value: &bypasses_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAction".into(),
                    value: &default_action_binding,
                },
                register_interface::ObjectField {
                    name: "ipRules".into(),
                    value: &ip_rules_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkAccessRules".into(),
                    value: &private_link_access_rules_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkSubnetIds".into(),
                    value: &virtual_network_subnet_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AccountNetworkRulesResult {
            bypasses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bypasses"),
            ),
            default_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultAction"),
            ),
            ip_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipRules"),
            ),
            private_link_access_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkAccessRules"),
            ),
            storage_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountId"),
            ),
            virtual_network_subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualNetworkSubnetIds"),
            ),
        }
    }
}
