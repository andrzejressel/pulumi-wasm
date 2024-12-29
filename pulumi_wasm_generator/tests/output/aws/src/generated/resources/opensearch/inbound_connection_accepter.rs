/// Manages an [AWS Opensearch Inbound Connection Accepter](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_AcceptInboundConnection.html). If connecting domains from different AWS accounts, ensure that the accepter is configured to use the AWS account where the _remote_ opensearch domain exists.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let currentGetRegion = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let foo = outbound_connection::create(
///         "foo",
///         OutboundConnectionArgs::builder()
///             .connection_alias("outbound_connection")
///             .local_domain_info(
///                 OutboundConnectionLocalDomainInfo::builder()
///                     .domainName("${localDomain.domainName}")
///                     .ownerId("${current.accountId}")
///                     .region("${currentGetRegion.name}")
///                     .build_struct(),
///             )
///             .remote_domain_info(
///                 OutboundConnectionRemoteDomainInfo::builder()
///                     .domainName("${remoteDomain.domainName}")
///                     .ownerId("${current.accountId}")
///                     .region("${currentGetRegion.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let fooInboundConnectionAccepter = inbound_connection_accepter::create(
///         "fooInboundConnectionAccepter",
///         InboundConnectionAccepterArgs::builder()
///             .connection_id("${foo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Opensearch Inbound Connection Accepters using the Inbound Connection ID. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/inboundConnectionAccepter:InboundConnectionAccepter foo connection-id
/// ```
pub mod inbound_connection_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InboundConnectionAccepterArgs {
        /// Specifies the ID of the connection to accept.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InboundConnectionAccepterResult {
        /// Specifies the ID of the connection to accept.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// Status of the connection request.
        pub connection_status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InboundConnectionAccepterArgs,
    ) -> InboundConnectionAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_id_binding = args.connection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/inboundConnectionAccepter:InboundConnectionAccepter"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "connectionStatus".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InboundConnectionAccepterResult {
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            connection_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionStatus").unwrap(),
            ),
        }
    }
}
