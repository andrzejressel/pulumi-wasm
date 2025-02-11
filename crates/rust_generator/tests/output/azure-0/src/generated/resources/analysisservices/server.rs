/// Manages an Analysis Services Server.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: analysis-services-server-test
///       location: West Europe
///   server:
///     type: azure:analysisservices:Server
///     properties:
///       name: analysisservicesserver
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: S0
///       adminUsers:
///         - myuser@domain.tld
///       powerBiServiceEnabled: true
///       ipv4FirewallRules:
///         - name: myRule1
///           rangeStart: 210.117.252.0
///           rangeEnd: 210.117.252.255
///       tags:
///         abc: 123
/// ```
///
/// > **NOTE:** The server resource will automatically be started and stopped during an update if it is in `paused` state.
///
/// ## Import
///
/// Analysis Services Server can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:analysisservices/server:Server server /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourcegroup1/providers/Microsoft.AnalysisServices/servers/server1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// List of email addresses of admin users.
        #[builder(into, default)]
        pub admin_users: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// URI and SAS token for a blob container to store backups.
        #[builder(into, default)]
        pub backup_blob_container_uri: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// One or more `ipv4_firewall_rule` block(s) as defined below.
        #[builder(into, default)]
        pub ipv4_firewall_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::analysisservices::ServerIpv4FirewallRule>>,
        >,
        /// The Azure location where the Analysis Services Server exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Analysis Services Server. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicates if the Power BI service is allowed to access or not.
        #[builder(into, default)]
        pub power_bi_service_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Controls how the read-write server is used in the query pool. If this value is set to `All` then read-write servers are also used for queries. Otherwise with `ReadOnly` these servers do not participate in query operations. Defaults to `All`.
        #[builder(into, default)]
        pub querypool_connection_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Resource Group in which the Analysis Services Server should be exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SKU for the Analysis Services Server. Possible values are: `D1`, `B1`, `B2`, `S0`, `S1`, `S2`, `S4`, `S8`, `S9`, `S8v2` and `S9v2`.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// List of email addresses of admin users.
        pub admin_users: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// URI and SAS token for a blob container to store backups.
        pub backup_blob_container_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `ipv4_firewall_rule` block(s) as defined below.
        pub ipv4_firewall_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::analysisservices::ServerIpv4FirewallRule>>,
        >,
        /// The Azure location where the Analysis Services Server exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Analysis Services Server. Only lowercase Alphanumeric characters allowed, starting with a letter. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Indicates if the Power BI service is allowed to access or not.
        pub power_bi_service_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Controls how the read-write server is used in the query pool. If this value is set to `All` then read-write servers are also used for queries. Otherwise with `ReadOnly` these servers do not participate in query operations. Defaults to `All`.
        pub querypool_connection_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which the Analysis Services Server should be exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The full name of the Analysis Services Server.
        pub server_full_name: pulumi_gestalt_rust::Output<String>,
        /// SKU for the Analysis Services Server. Possible values are: `D1`, `B1`, `B2`, `S0`, `S1`, `S2`, `S4`, `S8`, `S9`, `S8v2` and `S9v2`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerArgs,
    ) -> ServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let admin_users_binding = args.admin_users.get_output(context);
        let backup_blob_container_uri_binding = args
            .backup_blob_container_uri
            .get_output(context);
        let ipv4_firewall_rules_binding = args.ipv4_firewall_rules.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let power_bi_service_enabled_binding = args
            .power_bi_service_enabled
            .get_output(context);
        let querypool_connection_mode_binding = args
            .querypool_connection_mode
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:analysisservices/server:Server".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "adminUsers".into(),
                    value: &admin_users_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupBlobContainerUri".into(),
                    value: &backup_blob_container_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipv4FirewallRules".into(),
                    value: &ipv4_firewall_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "powerBiServiceEnabled".into(),
                    value: &power_bi_service_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "querypoolConnectionMode".into(),
                    value: &querypool_connection_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerResult {
            admin_users: o.get_field("adminUsers"),
            backup_blob_container_uri: o.get_field("backupBlobContainerUri"),
            ipv4_firewall_rules: o.get_field("ipv4FirewallRules"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            power_bi_service_enabled: o.get_field("powerBiServiceEnabled"),
            querypool_connection_mode: o.get_field("querypoolConnectionMode"),
            resource_group_name: o.get_field("resourceGroupName"),
            server_full_name: o.get_field("serverFullName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
