/// Provides a regional public access block for AMIs. This prevents AMIs from being made publicly accessible.
/// If you already have public AMIs, they will remain publicly available.
///
/// > **NOTE:** Deleting this resource does not change the block public access value, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = image_block_public_access::create(
///         "test",
///         ImageBlockPublicAccessArgs::builder().state("block-new-sharing").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image_block_public_access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageBlockPublicAccessArgs {
        /// The state of block public access for AMIs at the account level in the configured AWS Region. Valid values: `unblocked` and `block-new-sharing`.
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ImageBlockPublicAccessResult {
        /// The state of block public access for AMIs at the account level in the configured AWS Region. Valid values: `unblocked` and `block-new-sharing`.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ImageBlockPublicAccessArgs,
    ) -> ImageBlockPublicAccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let state_binding_1 = args.state.get_output(context);
        let state_binding = state_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/imageBlockPublicAccess:ImageBlockPublicAccess".into(),
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
        ImageBlockPublicAccessResult {
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
