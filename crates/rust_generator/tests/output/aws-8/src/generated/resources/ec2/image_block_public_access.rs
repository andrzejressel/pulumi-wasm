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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageBlockPublicAccessArgs,
    ) -> ImageBlockPublicAccessResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/imageBlockPublicAccess:ImageBlockPublicAccess".into(),
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
        ImageBlockPublicAccessResult {
            state: o.get_field("state"),
        }
    }
}
