pub mod get_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// Sha256 digest of the image manifest. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub image_digest: pulumi_wasm_rust::Output<Option<String>>,
        /// Tag associated with this image. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub image_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// Return the most recently pushed image. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// ID of the Registry where the repository resides.
        #[builder(into, default)]
        pub registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the ECR Repository.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_digest: pulumi_wasm_rust::Output<String>,
        /// Date and time, expressed as a unix timestamp, at which the current image was pushed to the repository.
        pub image_pushed_at: pulumi_wasm_rust::Output<i32>,
        /// Size, in bytes, of the image in the repository.
        pub image_size_in_bytes: pulumi_wasm_rust::Output<i32>,
        pub image_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// List of tags associated with this image.
        pub image_tags: pulumi_wasm_rust::Output<Vec<String>>,
        /// The URI for the specific image version specified by `image_tag` or `image_digest`.
        pub image_uri: pulumi_wasm_rust::Output<String>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        pub registry_id: pulumi_wasm_rust::Output<String>,
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetImageArgs) -> GetImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let image_digest_binding = args.image_digest.get_inner();
        let image_tag_binding = args.image_tag.get_inner();
        let most_recent_binding = args.most_recent.get_inner();
        let registry_id_binding = args.registry_id.get_inner();
        let repository_name_binding = args.repository_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getImage:getImage".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "imageDigest".into(),
                    value: &image_digest_binding,
                },
                register_interface::ObjectField {
                    name: "imageTag".into(),
                    value: &image_tag_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageDigest".into(),
                },
                register_interface::ResultField {
                    name: "imagePushedAt".into(),
                },
                register_interface::ResultField {
                    name: "imageSizeInBytes".into(),
                },
                register_interface::ResultField {
                    name: "imageTag".into(),
                },
                register_interface::ResultField {
                    name: "imageTags".into(),
                },
                register_interface::ResultField {
                    name: "imageUri".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repositoryName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetImageResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageDigest").unwrap(),
            ),
            image_pushed_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imagePushedAt").unwrap(),
            ),
            image_size_in_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageSizeInBytes").unwrap(),
            ),
            image_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTag").unwrap(),
            ),
            image_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageTags").unwrap(),
            ),
            image_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageUri").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repositoryName").unwrap(),
            ),
        }
    }
}
