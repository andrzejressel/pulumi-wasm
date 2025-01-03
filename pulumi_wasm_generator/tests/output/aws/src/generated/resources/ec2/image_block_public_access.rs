/// Provides a regional public access block for AMIs. This prevents AMIs from being made publicly accessible.
/// If you already have public AMIs, they will remain publicly available.
///
/// > **NOTE:** Deleting this resource does not change the block public access value, the resource in simply removed from state instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod image_block_public_access {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageBlockPublicAccessArgs {
        /// The state of block public access for AMIs at the account level in the configured AWS Region. Valid values: `unblocked` and `block-new-sharing`.
        #[builder(into)]
        pub state: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ImageBlockPublicAccessResult {
        /// The state of block public access for AMIs at the account level in the configured AWS Region. Valid values: `unblocked` and `block-new-sharing`.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ImageBlockPublicAccessArgs,
    ) -> ImageBlockPublicAccessResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let state_binding = args.state.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/imageBlockPublicAccess:ImageBlockPublicAccess".into(),
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
        ImageBlockPublicAccessResult {
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
