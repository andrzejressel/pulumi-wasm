/// Manages a Log Analytics Cluster Customer Managed Key.
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
///   exampleCluster:
///     type: azure:loganalytics:Cluster
///     name: example
///     properties:
///       name: example-cluster
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: keyvaultkeyexample
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           keyPermissions:
///             - Create
///             - Get
///             - GetRotationPolicy
///           secretPermissions:
///             - Set
///         - tenantId: ${exampleCluster.identity.tenantId}
///           objectId: ${exampleCluster.identity.principalId}
///           keyPermissions:
///             - Get
///             - Unwrapkey
///             - Wrapkey
///       tags:
///         environment: Production
///   exampleKey:
///     type: azure:keyvault:Key
///     name: example
///     properties:
///       name: generated-certificate
///       keyVaultId: ${exampleKeyVault.id}
///       keyType: RSA
///       keySize: 2048
///       keyOpts:
///         - decrypt
///         - encrypt
///         - sign
///         - unwrapKey
///         - verify
///         - wrapKey
///   exampleClusterCustomerManagedKey:
///     type: azure:loganalytics:ClusterCustomerManagedKey
///     name: example
///     properties:
///       logAnalyticsClusterId: ${exampleCluster.id}
///       keyVaultKeyId: ${exampleKey.id}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Log Analytics Cluster Customer Managed Keys can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:loganalytics/clusterCustomerManagedKey:ClusterCustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_customer_managed_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterCustomerManagedKeyArgs {
        /// The ID of the Key Vault Key to use for encryption.
        #[builder(into)]
        pub key_vault_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Log Analytics Cluster. Changing this forces a new Log Analytics Cluster Customer Managed Key to be created.
        #[builder(into)]
        pub log_analytics_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterCustomerManagedKeyResult {
        /// The ID of the Key Vault Key to use for encryption.
        pub key_vault_key_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Log Analytics Cluster. Changing this forces a new Log Analytics Cluster Customer Managed Key to be created.
        pub log_analytics_cluster_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterCustomerManagedKeyArgs,
    ) -> ClusterCustomerManagedKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let key_vault_key_id_binding = args
            .key_vault_key_id
            .get_output(context)
            .get_inner();
        let log_analytics_cluster_id_binding = args
            .log_analytics_cluster_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:loganalytics/clusterCustomerManagedKey:ClusterCustomerManagedKey"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyVaultKeyId".into(),
                    value: &key_vault_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logAnalyticsClusterId".into(),
                    value: &log_analytics_cluster_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterCustomerManagedKeyResult {
            key_vault_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyVaultKeyId"),
            ),
            log_analytics_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsClusterId"),
            ),
        }
    }
}
