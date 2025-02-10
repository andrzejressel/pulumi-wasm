#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_docker_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDockerImageArgs {
        /// The image name to fetch. If no digest or tag is provided, then the latest modified image will be used.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the artifact registry.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project ID in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The last part of the repository name to fetch from.
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDockerImageResult {
        /// The time, as a RFC 3339 string, this image was built.
        pub build_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// Calculated size of the image in bytes.
        pub image_size_bytes: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Media type of this image, e.g. `application/vnd.docker.distribution.manifest.v2+json`.
        pub media_type: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified name of the fetched image.  This name has the form: `projects/{{project}}/locations/{{location}}/repository/{{repository_id}}/dockerImages/{{docker_image}}`. For example,
        /// ```sh
        /// projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
        /// ```
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        /// The URI to access the image.  For example,
        /// ```sh
        /// us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
        /// ```
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A list of all tags associated with the image.
        pub tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The time, as a RFC 3339 string, this image was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The time, as a RFC 3339 string, the image was uploaded. For example, `2014-10-02T15:01:23.045123456Z`.
        pub upload_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDockerImageArgs,
    ) -> GetDockerImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let image_name_binding = args.image_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let repository_id_binding = args.repository_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:artifactregistry/getDockerImage:getDockerImage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageName".into(),
                    value: image_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryId".into(),
                    value: repository_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDockerImageResult {
            build_time: o.get_field("buildTime"),
            id: o.get_field("id"),
            image_name: o.get_field("imageName"),
            image_size_bytes: o.get_field("imageSizeBytes"),
            location: o.get_field("location"),
            media_type: o.get_field("mediaType"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            repository_id: o.get_field("repositoryId"),
            self_link: o.get_field("selfLink"),
            tags: o.get_field("tags"),
            update_time: o.get_field("updateTime"),
            upload_time: o.get_field("uploadTime"),
        }
    }
}
