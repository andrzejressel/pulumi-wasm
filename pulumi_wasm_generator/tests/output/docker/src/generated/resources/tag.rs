/// Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
pub mod tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Name of the source image.
        #[builder(into)]
        pub source_image: pulumi_wasm_rust::Output<String>,
        /// Name of the target image.
        #[builder(into)]
        pub target_image: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: TagArgs) -> TagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let source_image_binding = args.source_image.get_inner();
        let target_image_binding = args.target_image.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "sourceImage".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "targetImage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagResult {
            source_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImage").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            target_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetImage").unwrap(),
            ),
        }
    }
}
