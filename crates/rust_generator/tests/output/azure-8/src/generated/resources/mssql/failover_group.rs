/// Manages a Microsoft Azure SQL Failover Group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: database-rg
///       location: West Europe
///   primary:
///     type: azure:mssql:Server
///     properties:
///       name: mssqlserver-primary
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       version: '12.0'
///       administratorLogin: missadministrator
///       administratorLoginPassword: thisIsKat11
///   secondary:
///     type: azure:mssql:Server
///     properties:
///       name: mssqlserver-secondary
///       resourceGroupName: ${example.name}
///       location: North Europe
///       version: '12.0'
///       administratorLogin: missadministrator
///       administratorLoginPassword: thisIsKat12
///   exampleDatabase:
///     type: azure:mssql:Database
///     name: example
///     properties:
///       name: exampledb
///       serverId: ${primary.id}
///       skuName: S1
///       collation: SQL_Latin1_General_CP1_CI_AS
///       maxSizeGb: '200'
///   exampleFailoverGroup:
///     type: azure:mssql:FailoverGroup
///     name: example
///     properties:
///       name: example
///       serverId: ${primary.id}
///       databases:
///         - ${exampleDatabase.id}
///       partnerServers:
///         - id: ${secondary.id}
///       readWriteEndpointFailoverPolicy:
///         mode: Automatic
///         graceMinutes: 80
///       tags:
///         environment: prod
///         database: example
/// ```
///
/// ## Import
///
/// Failover Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/failoverGroup:FailoverGroup example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Sql/servers/server1/failoverGroups/failoverGroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod failover_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FailoverGroupArgs {
        /// A set of database names to include in the failover group.
        #[builder(into, default)]
        pub databases: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Failover Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `partner_server` block as defined below.
        #[builder(into)]
        pub partner_servers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::mssql::FailoverGroupPartnerServer>,
        >,
        /// A `read_write_endpoint_failover_policy` block as defined below.
        #[builder(into)]
        pub read_write_endpoint_failover_policy: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mssql::FailoverGroupReadWriteEndpointFailoverPolicy,
        >,
        /// Whether failover is enabled for the readonly endpoint. Defaults to `false`.
        #[builder(into, default)]
        pub readonly_endpoint_failover_policy_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the primary SQL Server on which to create the failover group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FailoverGroupResult {
        /// A set of database names to include in the failover group.
        pub databases: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Failover Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `partner_server` block as defined below.
        pub partner_servers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mssql::FailoverGroupPartnerServer>,
        >,
        /// A `read_write_endpoint_failover_policy` block as defined below.
        pub read_write_endpoint_failover_policy: pulumi_gestalt_rust::Output<
            super::super::types::mssql::FailoverGroupReadWriteEndpointFailoverPolicy,
        >,
        /// Whether failover is enabled for the readonly endpoint. Defaults to `false`.
        pub readonly_endpoint_failover_policy_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The ID of the primary SQL Server on which to create the failover group. Changing this forces a new resource to be created.
        pub server_id: pulumi_gestalt_rust::Output<String>,
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
        args: FailoverGroupArgs,
    ) -> FailoverGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let databases_binding = args.databases.get_output(context);
        let name_binding = args.name.get_output(context);
        let partner_servers_binding = args.partner_servers.get_output(context);
        let read_write_endpoint_failover_policy_binding = args
            .read_write_endpoint_failover_policy
            .get_output(context);
        let readonly_endpoint_failover_policy_enabled_binding = args
            .readonly_endpoint_failover_policy_enabled
            .get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/failoverGroup:FailoverGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databases".into(),
                    value: &databases_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partnerServers".into(),
                    value: &partner_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readWriteEndpointFailoverPolicy".into(),
                    value: &read_write_endpoint_failover_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readonlyEndpointFailoverPolicyEnabled".into(),
                    value: &readonly_endpoint_failover_policy_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: &server_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FailoverGroupResult {
            databases: o.get_field("databases"),
            name: o.get_field("name"),
            partner_servers: o.get_field("partnerServers"),
            read_write_endpoint_failover_policy: o
                .get_field("readWriteEndpointFailoverPolicy"),
            readonly_endpoint_failover_policy_enabled: o
                .get_field("readonlyEndpointFailoverPolicyEnabled"),
            server_id: o.get_field("serverId"),
            tags: o.get_field("tags"),
        }
    }
}
