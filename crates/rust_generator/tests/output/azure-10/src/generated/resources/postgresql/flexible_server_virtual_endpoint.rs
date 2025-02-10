/// Allows you to create a Virtual Endpoint associated with a Postgres Flexible Replica.
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
///             .location("East US")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFlexibleServer = flexible_server::create(
///         "exampleFlexibleServer",
///         FlexibleServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_password("H@Sh1CoR3!")
///             .location("${example.location}")
///             .name("example")
///             .public_network_access_enabled(false)
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Standard_D2ads_v5")
///             .storage_mb(32768)
///             .storage_tier("P30")
///             .version("16")
///             .zone("1")
///             .build_struct(),
///     );
///     let exampleFlexibleServerVirtualEndpoint = flexible_server_virtual_endpoint::create(
///         "exampleFlexibleServerVirtualEndpoint",
///         FlexibleServerVirtualEndpointArgs::builder()
///             .name("example-endpoint-1")
///             .replica_server_id("${exampleReplica.id}")
///             .source_server_id("${exampleFlexibleServer.id}")
///             .type_("ReadWrite")
///             .build_struct(),
///     );
///     let exampleReplica = flexible_server::create(
///         "exampleReplica",
///         FlexibleServerArgs::builder()
///             .create_mode("Replica")
///             .location("${exampleFlexibleServer.location}")
///             .name("example-replica")
///             .public_network_access_enabled(false)
///             .resource_group_name("${exampleFlexibleServer.resourceGroupName}")
///             .sku_name("GP_Standard_D2ads_v5")
///             .source_server_id("${exampleFlexibleServer.id}")
///             .storage_mb(32768)
///             .storage_tier("P30")
///             .version("16")
///             .zone("1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// > **Note:** If creating multiple replicas, an error can occur if virtual endpoints are created before all replicas have been completed. To avoid this error, use a `depends_on` property on `azure.postgresql.FlexibleServerVirtualEndpoint` that references all Postgres Flexible Server Replicas.
///
/// ## Import
///
/// A PostgreSQL Flexible Virtual Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/flexibleServerVirtualEndpoint:FlexibleServerVirtualEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DBforPostgreSQL/flexibleServers/server1/virtualEndpoints/endpoint1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod flexible_server_virtual_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlexibleServerVirtualEndpointArgs {
        /// The name of the Virtual Endpoint
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Resource ID of the *Replica* Postgres Flexible Server this should be associated with
        #[builder(into)]
        pub replica_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Resource ID of the *Source* Postgres Flexible Server this should be associated with.
        #[builder(into)]
        pub source_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of Virtual Endpoint. Currently only `ReadWrite` is supported.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FlexibleServerVirtualEndpointResult {
        /// The name of the Virtual Endpoint
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Resource ID of the *Replica* Postgres Flexible Server this should be associated with
        pub replica_server_id: pulumi_gestalt_rust::Output<String>,
        /// The Resource ID of the *Source* Postgres Flexible Server this should be associated with.
        pub source_server_id: pulumi_gestalt_rust::Output<String>,
        /// The type of Virtual Endpoint. Currently only `ReadWrite` is supported.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FlexibleServerVirtualEndpointArgs,
    ) -> FlexibleServerVirtualEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let replica_server_id_binding = args.replica_server_id.get_output(context);
        let source_server_id_binding = args.source_server_id.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:postgresql/flexibleServerVirtualEndpoint:FlexibleServerVirtualEndpoint"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicaServerId".into(),
                    value: replica_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceServerId".into(),
                    value: source_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FlexibleServerVirtualEndpointResult {
            name: o.get_field("name"),
            replica_server_id: o.get_field("replicaServerId"),
            source_server_id: o.get_field("sourceServerId"),
            type_: o.get_field("type"),
        }
    }
}
