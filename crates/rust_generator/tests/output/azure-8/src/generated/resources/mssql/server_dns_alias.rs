/// Manages a MS SQL Server DNS Alias.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("missadministrator")
///             .administrator_login_password("AdminPassword123!")
///             .location("${example.location}")
///             .name("example-sqlserver")
///             .resource_group_name("${example.name}")
///             .version("12.0")
///             .build_struct(),
///     );
///     let exampleServerDnsAlias = server_dns_alias::create(
///         "exampleServerDnsAlias",
///         ServerDnsAliasArgs::builder()
///             .mssql_server_id("${exampleServer.id}")
///             .name("example-dns-alias")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// MSSQL Server DNS Aliass can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/serverDnsAlias:ServerDnsAlias example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Sql/servers/server1/dnsAliases/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod server_dns_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerDnsAliasArgs {
        /// The ID of the mssql server. Changing this forces a new MSSQL Server DNS Alias to be created.
        #[builder(into)]
        pub mssql_server_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this MSSQL Server DNS Alias. Changing this forces a new MSSQL Server DNS Alias to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServerDnsAliasResult {
        /// The fully qualified DNS record for alias.
        pub dns_record: pulumi_gestalt_rust::Output<String>,
        /// The ID of the mssql server. Changing this forces a new MSSQL Server DNS Alias to be created.
        pub mssql_server_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this MSSQL Server DNS Alias. Changing this forces a new MSSQL Server DNS Alias to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerDnsAliasArgs,
    ) -> ServerDnsAliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mssql_server_id_binding = args.mssql_server_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/serverDnsAlias:ServerDnsAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mssqlServerId".into(),
                    value: mssql_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerDnsAliasResult {
            dns_record: o.get_field("dnsRecord"),
            mssql_server_id: o.get_field("mssqlServerId"),
            name: o.get_field("name"),
        }
    }
}
