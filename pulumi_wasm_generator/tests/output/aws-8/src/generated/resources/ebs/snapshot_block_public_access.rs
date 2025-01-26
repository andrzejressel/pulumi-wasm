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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotBlockPublicAccessArgs {
        /// The mode in which to enable "Block public access for snapshots" for the region. Allowed values are `block-all-sharing`, `block-new-sharing`, `unblocked`.
        #[builder(into)]
        pub state: pulumi_wasm_rust::InputOrOutput<String>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SnapshotBlockPublicAccessArgs,
    ) -> SnapshotBlockPublicAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let state_binding = args.state.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/snapshotBlockPublicAccess:SnapshotBlockPublicAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SnapshotBlockPublicAccessResult {
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
