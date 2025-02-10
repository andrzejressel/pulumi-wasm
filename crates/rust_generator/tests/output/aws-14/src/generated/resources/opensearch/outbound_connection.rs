/// Manages an AWS Opensearch Outbound Connection.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:opensearch:OutboundConnection
///     properties:
///       connectionAlias: outbound_connection
///       connectionMode: DIRECT
///       localDomainInfo:
///         ownerId: ${current.accountId}
///         region: ${currentGetRegion.name}
///         domainName: ${localDomain.domainName}
///       remoteDomainInfo:
///         ownerId: ${current.accountId}
///         region: ${currentGetRegion.name}
///         domainName: ${remoteDomain.domainName}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Opensearch Outbound Connections using the Outbound Connection ID. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/outboundConnection:OutboundConnection foo connection-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod outbound_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutboundConnectionArgs {
        /// Accepts the connection.
        #[builder(into, default)]
        pub accept_connection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the connection alias that will be used by the customer for this connection.
        #[builder(into)]
        pub connection_alias: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the connection mode. Accepted values are `DIRECT` or `VPC_ENDPOINT`.
        #[builder(into, default)]
        pub connection_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the outbound connection.
        #[builder(into, default)]
        pub connection_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::opensearch::OutboundConnectionConnectionProperties,
            >,
        >,
        /// Configuration block for the local Opensearch domain.
        #[builder(into)]
        pub local_domain_info: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::opensearch::OutboundConnectionLocalDomainInfo,
        >,
        /// Configuration block for the remote Opensearch domain.
        #[builder(into)]
        pub remote_domain_info: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::opensearch::OutboundConnectionRemoteDomainInfo,
        >,
    }
    #[allow(dead_code)]
    pub struct OutboundConnectionResult {
        /// Accepts the connection.
        pub accept_connection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the connection alias that will be used by the customer for this connection.
        pub connection_alias: pulumi_gestalt_rust::Output<String>,
        /// Specifies the connection mode. Accepted values are `DIRECT` or `VPC_ENDPOINT`.
        pub connection_mode: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the outbound connection.
        pub connection_properties: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::OutboundConnectionConnectionProperties,
        >,
        /// Status of the connection request.
        pub connection_status: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the local Opensearch domain.
        pub local_domain_info: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::OutboundConnectionLocalDomainInfo,
        >,
        /// Configuration block for the remote Opensearch domain.
        pub remote_domain_info: pulumi_gestalt_rust::Output<
            super::super::types::opensearch::OutboundConnectionRemoteDomainInfo,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OutboundConnectionArgs,
    ) -> OutboundConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accept_connection_binding = args.accept_connection.get_output(context);
        let connection_alias_binding = args.connection_alias.get_output(context);
        let connection_mode_binding = args.connection_mode.get_output(context);
        let connection_properties_binding = args
            .connection_properties
            .get_output(context);
        let local_domain_info_binding = args.local_domain_info.get_output(context);
        let remote_domain_info_binding = args.remote_domain_info.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/outboundConnection:OutboundConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceptConnection".into(),
                    value: accept_connection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionAlias".into(),
                    value: connection_alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionMode".into(),
                    value: connection_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionProperties".into(),
                    value: connection_properties_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localDomainInfo".into(),
                    value: local_domain_info_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteDomainInfo".into(),
                    value: remote_domain_info_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OutboundConnectionResult {
            accept_connection: o.get_field("acceptConnection"),
            connection_alias: o.get_field("connectionAlias"),
            connection_mode: o.get_field("connectionMode"),
            connection_properties: o.get_field("connectionProperties"),
            connection_status: o.get_field("connectionStatus"),
            local_domain_info: o.get_field("localDomainInfo"),
            remote_domain_info: o.get_field("remoteDomainInfo"),
        }
    }
}
