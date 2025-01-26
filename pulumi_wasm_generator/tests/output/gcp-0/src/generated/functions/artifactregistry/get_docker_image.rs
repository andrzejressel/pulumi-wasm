pub mod get_docker_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDockerImageArgs {
        /// The image name to fetch. If no digest or tag is provided, then the latest modified image will be used.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the artifact registry.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project ID in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The last part of the repository name to fetch from.
        #[builder(into)]
        pub repository_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDockerImageResult {
        /// The time, as a RFC 3339 string, this image was built.
        pub build_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// Calculated size of the image in bytes.
        pub image_size_bytes: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// Media type of this image, e.g. `application/vnd.docker.distribution.manifest.v2+json`.
        pub media_type: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of the fetched image.  This name has the form: `projects/{{project}}/locations/{{location}}/repository/{{repository_id}}/dockerImages/{{docker_image}}`. For example,
        /// ```sh
        /// projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
        /// ```
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub repository_id: pulumi_wasm_rust::Output<String>,
        /// The URI to access the image.  For example,
        /// ```sh
        /// us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
        /// ```
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// A list of all tags associated with the image.
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        /// The time, as a RFC 3339 string, this image was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The time, as a RFC 3339 string, the image was uploaded. For example, `2014-10-02T15:01:23.045123456Z`.
        pub upload_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDockerImageArgs,
    ) -> GetDockerImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let image_name_binding = args.image_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repository_id_binding = args.repository_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:artifactregistry/getDockerImage:getDockerImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDockerImageResult {
            build_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("buildTime"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            image_size_bytes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageSizeBytes"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            media_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mediaType"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            upload_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("uploadTime"),
            ),
        }
    }
}
