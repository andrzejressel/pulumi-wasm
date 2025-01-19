pub mod get_registry_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryImageArgs {
        /// The image digest to fetch, if any.
        #[builder(into, default)]
        pub digest: pulumi_wasm_rust::Output<Option<String>>,
        /// The image name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project ID that this image is attached to.  If not provider, provider project will be used instead.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The GCR region to use.  As of this writing, one of `asia`, `eu`, and `us`.  See [the documentation](https://cloud.google.com/container-registry/docs/pushing-and-pulling) for additional information.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The tag to fetch, if any.
        #[builder(into, default)]
        pub tag: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryImageResult {
        pub digest: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The URL at which the image can be accessed.
        pub image_url: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        pub tag: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRegistryImageArgs) -> GetRegistryImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let digest_binding = args.digest.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let tag_binding = args.tag.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getRegistryImage:getRegistryImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "digest".into(),
                    value: &digest_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "tag".into(),
                    value: &tag_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "digest".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "imageUrl".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "tag".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRegistryImageResult {
            digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digest").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            image_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageUrl").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            tag: pulumi_wasm_rust::__private::into_domain(hashmap.remove("tag").unwrap()),
        }
    }
}
