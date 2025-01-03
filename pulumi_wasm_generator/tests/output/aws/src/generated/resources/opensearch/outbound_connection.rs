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
pub mod outbound_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutboundConnectionArgs {
        /// Accepts the connection.
        #[builder(into, default)]
        pub accept_connection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the connection alias that will be used by the customer for this connection.
        #[builder(into)]
        pub connection_alias: pulumi_wasm_rust::Output<String>,
        /// Specifies the connection mode. Accepted values are `DIRECT` or `VPC_ENDPOINT`.
        #[builder(into, default)]
        pub connection_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for the outbound connection.
        #[builder(into, default)]
        pub connection_properties: pulumi_wasm_rust::Output<
            Option<
                super::super::types::opensearch::OutboundConnectionConnectionProperties,
            >,
        >,
        /// Configuration block for the local Opensearch domain.
        #[builder(into)]
        pub local_domain_info: pulumi_wasm_rust::Output<
            super::super::types::opensearch::OutboundConnectionLocalDomainInfo,
        >,
        /// Configuration block for the remote Opensearch domain.
        #[builder(into)]
        pub remote_domain_info: pulumi_wasm_rust::Output<
            super::super::types::opensearch::OutboundConnectionRemoteDomainInfo,
        >,
    }
    #[allow(dead_code)]
    pub struct OutboundConnectionResult {
        /// Accepts the connection.
        pub accept_connection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the connection alias that will be used by the customer for this connection.
        pub connection_alias: pulumi_wasm_rust::Output<String>,
        /// Specifies the connection mode. Accepted values are `DIRECT` or `VPC_ENDPOINT`.
        pub connection_mode: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the outbound connection.
        pub connection_properties: pulumi_wasm_rust::Output<
            super::super::types::opensearch::OutboundConnectionConnectionProperties,
        >,
        /// Status of the connection request.
        pub connection_status: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the local Opensearch domain.
        pub local_domain_info: pulumi_wasm_rust::Output<
            super::super::types::opensearch::OutboundConnectionLocalDomainInfo,
        >,
        /// Configuration block for the remote Opensearch domain.
        pub remote_domain_info: pulumi_wasm_rust::Output<
            super::super::types::opensearch::OutboundConnectionRemoteDomainInfo,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OutboundConnectionArgs) -> OutboundConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_connection_binding = args.accept_connection.get_inner();
        let connection_alias_binding = args.connection_alias.get_inner();
        let connection_mode_binding = args.connection_mode.get_inner();
        let connection_properties_binding = args.connection_properties.get_inner();
        let local_domain_info_binding = args.local_domain_info.get_inner();
        let remote_domain_info_binding = args.remote_domain_info.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/outboundConnection:OutboundConnection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptConnection".into(),
                    value: &accept_connection_binding,
                },
                register_interface::ObjectField {
                    name: "connectionAlias".into(),
                    value: &connection_alias_binding,
                },
                register_interface::ObjectField {
                    name: "connectionMode".into(),
                    value: &connection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "connectionProperties".into(),
                    value: &connection_properties_binding,
                },
                register_interface::ObjectField {
                    name: "localDomainInfo".into(),
                    value: &local_domain_info_binding,
                },
                register_interface::ObjectField {
                    name: "remoteDomainInfo".into(),
                    value: &remote_domain_info_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptConnection".into(),
                },
                register_interface::ResultField {
                    name: "connectionAlias".into(),
                },
                register_interface::ResultField {
                    name: "connectionMode".into(),
                },
                register_interface::ResultField {
                    name: "connectionProperties".into(),
                },
                register_interface::ResultField {
                    name: "connectionStatus".into(),
                },
                register_interface::ResultField {
                    name: "localDomainInfo".into(),
                },
                register_interface::ResultField {
                    name: "remoteDomainInfo".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutboundConnectionResult {
            accept_connection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptConnection").unwrap(),
            ),
            connection_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionAlias").unwrap(),
            ),
            connection_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionMode").unwrap(),
            ),
            connection_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProperties").unwrap(),
            ),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStatus").unwrap(),
            ),
            local_domain_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localDomainInfo").unwrap(),
            ),
            remote_domain_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteDomainInfo").unwrap(),
            ),
        }
    }
}
