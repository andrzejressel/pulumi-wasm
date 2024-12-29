/// Associates a Direct Connect Connection with a LAG.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connection::create(
///         "example",
///         ConnectionArgs::builder()
///             .bandwidth("1Gbps")
///             .location("EqSe2-EQ")
///             .name("example")
///             .build_struct(),
///     );
///     let exampleConnectionAssociation = connection_association::create(
///         "exampleConnectionAssociation",
///         ConnectionAssociationArgs::builder()
///             .connection_id("${example.id}")
///             .lag_id("${exampleLinkAggregationGroup.id}")
///             .build_struct(),
///     );
///     let exampleLinkAggregationGroup = link_aggregation_group::create(
///         "exampleLinkAggregationGroup",
///         LinkAggregationGroupArgs::builder()
///             .connections_bandwidth("1Gbps")
///             .location("EqSe2-EQ")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
pub mod connection_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionAssociationArgs {
        /// The ID of the connection.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the LAG with which to associate the connection.
        #[builder(into)]
        pub lag_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionAssociationResult {
        /// The ID of the connection.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the LAG with which to associate the connection.
        pub lag_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConnectionAssociationArgs,
    ) -> ConnectionAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_id_binding = args.connection_id.get_inner();
        let lag_id_binding = args.lag_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/connectionAssociation:ConnectionAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "lagId".into(),
                    value: &lag_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "lagId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionAssociationResult {
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            lag_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lagId").unwrap(),
            ),
        }
    }
}
