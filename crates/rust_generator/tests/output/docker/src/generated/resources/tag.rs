/// Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tag {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Name of the source image.
        #[builder(into)]
        pub source_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the target image.
        #[builder(into)]
        pub target_image: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TagResult {
        /// Name of the source image.
        pub source_image: pulumi_gestalt_rust::Output<String>,
        /// ImageID of the source image in the format of `sha256:<<ID>>`
        pub source_image_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the target image.
        pub target_image: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TagArgs,
    ) -> TagResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let source_image_binding = args.source_image.get_output(context);
        let target_image_binding = args.target_image.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceImage".into(),
                    value: &source_image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetImage".into(),
                    value: &target_image_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TagResult {
            source_image: o.get_field("sourceImage"),
            source_image_id: o.get_field("sourceImageId"),
            target_image: o.get_field("targetImage"),
        }
    }
}
