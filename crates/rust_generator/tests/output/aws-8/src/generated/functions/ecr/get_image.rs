#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetImageArgs {
        /// Sha256 digest of the image manifest. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub image_digest: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tag associated with this image. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub image_tag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Return the most recently pushed image. At least one of `image_digest`, `image_tag`, or `most_recent` must be specified.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// ID of the Registry where the repository resides.
        #[builder(into, default)]
        pub registry_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the ECR Repository.
        #[builder(into)]
        pub repository_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetImageResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_digest: pulumi_gestalt_rust::Output<String>,
        /// Date and time, expressed as a unix timestamp, at which the current image was pushed to the repository.
        pub image_pushed_at: pulumi_gestalt_rust::Output<i32>,
        /// Size, in bytes, of the image in the repository.
        pub image_size_in_bytes: pulumi_gestalt_rust::Output<i32>,
        pub image_tag: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of tags associated with this image.
        pub image_tags: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The URI for the specific image version specified by `image_tag` or `image_digest`.
        pub image_uri: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        pub repository_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetImageArgs,
    ) -> GetImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let image_digest_binding = args.image_digest.get_output(context);
        let image_tag_binding = args.image_tag.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let registry_id_binding = args.registry_id.get_output(context);
        let repository_name_binding = args.repository_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ecr/getImage:getImage".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageDigest".into(),
                    value: &image_digest_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageTag".into(),
                    value: &image_tag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetImageResult {
            id: o.get_field("id"),
            image_digest: o.get_field("imageDigest"),
            image_pushed_at: o.get_field("imagePushedAt"),
            image_size_in_bytes: o.get_field("imageSizeInBytes"),
            image_tag: o.get_field("imageTag"),
            image_tags: o.get_field("imageTags"),
            image_uri: o.get_field("imageUri"),
            most_recent: o.get_field("mostRecent"),
            registry_id: o.get_field("registryId"),
            repository_name: o.get_field("repositoryName"),
        }
    }
}
