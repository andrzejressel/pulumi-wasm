/// Provides a confirmation of the creation of the specified hosted connection on an interconnect.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let confirmation = connection_confirmation::create(
///         "confirmation",
///         ConnectionConfirmationArgs::builder()
///             .connection_id("dxcon-ffabc123")
///             .build_struct(),
///     );
/// }
/// ```
pub mod connection_confirmation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionConfirmationArgs {
        /// The ID of the hosted connection.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectionConfirmationResult {
        /// The ID of the hosted connection.
        pub connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConnectionConfirmationArgs,
    ) -> ConnectionConfirmationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_id_binding = args.connection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/connectionConfirmation:ConnectionConfirmation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionConfirmationResult {
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
        }
    }
}
