/// Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
pub mod tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Name of the source image.
        #[builder(into)]
        pub source_image: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the target image.
        #[builder(into)]
        pub target_image: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// Name of the source image.
        pub source_image: pulumi_wasm_rust::Output<String>,
        /// ImageID of the source image in the format of `sha256:<<ID>>`
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        /// Name of the target image.
        pub target_image: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let source_image_binding = args.source_image.get_output(context).get_inner();
        let target_image_binding = args.target_image.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "sourceImage".into(),
                    value: &source_image_binding,
                },
                register_interface::ObjectField {
                    name: "targetImage".into(),
                    value: &target_image_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TagResult {
            source_image: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceImage"),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceImageId"),
            ),
            target_image: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetImage"),
            ),
        }
    }
}
