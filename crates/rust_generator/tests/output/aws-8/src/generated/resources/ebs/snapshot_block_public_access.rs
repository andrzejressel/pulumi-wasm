/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod snapshot_block_public_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SnapshotBlockPublicAccessArgs {
        /// The mode in which to enable "Block public access for snapshots" for the region. Allowed values are `block-all-sharing`, `block-new-sharing`, `unblocked`.
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SnapshotBlockPublicAccessResult {
        /// The mode in which to enable "Block public access for snapshots" for the region. Allowed values are `block-all-sharing`, `block-new-sharing`, `unblocked`.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SnapshotBlockPublicAccessArgs,
    ) -> SnapshotBlockPublicAccessResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ebs/snapshotBlockPublicAccess:SnapshotBlockPublicAccess".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SnapshotBlockPublicAccessResult {
            state: o.get_field("state"),
        }
    }
}
