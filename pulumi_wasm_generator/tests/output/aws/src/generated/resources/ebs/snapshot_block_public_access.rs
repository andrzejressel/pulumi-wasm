/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = snapshot_block_public_access::create(
///         "example",
///         SnapshotBlockPublicAccessArgs::builder()
///             .state("block-all-sharing")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the state. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/snapshotBlockPublicAccess:SnapshotBlockPublicAccess example default
/// ```
pub mod snapshot_block_public_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotBlockPublicAccessArgs {
        /// The mode in which to enable "Block public access for snapshots" for the region. Allowed values are `block-all-sharing`, `block-new-sharing`, `unblocked`.
        #[builder(into)]
        pub state: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotBlockPublicAccessResult {
        /// The mode in which to enable "Block public access for snapshots" for the region. Allowed values are `block-all-sharing`, `block-new-sharing`, `unblocked`.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SnapshotBlockPublicAccessArgs,
    ) -> SnapshotBlockPublicAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let state_binding = args.state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/snapshotBlockPublicAccess:SnapshotBlockPublicAccess".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SnapshotBlockPublicAccessResult {
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
