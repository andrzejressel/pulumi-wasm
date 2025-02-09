/// Manages an [AWS Opensearch Inbound Connection Accepter](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_AcceptInboundConnection.html). If connecting domains from different AWS accounts, ensure that the accepter is configured to use the AWS account where the _remote_ opensearch domain exists.
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
///       localDomainInfo:
///         ownerId: ${current.accountId}
///         region: ${currentGetRegion.name}
///         domainName: ${localDomain.domainName}
///       remoteDomainInfo:
///         ownerId: ${current.accountId}
///         region: ${currentGetRegion.name}
///         domainName: ${remoteDomain.domainName}
///   fooInboundConnectionAccepter:
///     type: aws:opensearch:InboundConnectionAccepter
///     name: foo
///     properties:
///       connectionId: ${foo.id}
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
/// Using `pulumi import`, import AWS Opensearch Inbound Connection Accepters using the Inbound Connection ID. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/inboundConnectionAccepter:InboundConnectionAccepter foo connection-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod inbound_connection_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InboundConnectionAccepterArgs {
        /// Specifies the ID of the connection to accept.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InboundConnectionAccepterResult {
        /// Specifies the ID of the connection to accept.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// Status of the connection request.
        pub connection_status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InboundConnectionAccepterArgs,
    ) -> InboundConnectionAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_id_binding = args.connection_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/inboundConnectionAccepter:InboundConnectionAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: connection_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InboundConnectionAccepterResult {
            connection_id: o.get_field("connectionId"),
            connection_status: o.get_field("connectionStatus"),
        }
    }
}
