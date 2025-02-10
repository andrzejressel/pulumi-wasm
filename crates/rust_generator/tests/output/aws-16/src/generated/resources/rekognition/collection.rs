/// Resource for managing an AWS Rekognition Collection.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rekognition:Collection
///     properties:
///       collectionId: my-collection
///       tags:
///         example: 1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Rekognition Collection using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:rekognition/collection:Collection example collection-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CollectionArgs {
        /// The name of the collection
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub collection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::CollectionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct CollectionResult {
        /// ARN of the Collection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the collection
        ///
        /// The following arguments are optional:
        pub collection_id: pulumi_gestalt_rust::Output<String>,
        /// The Face Model Version that the collection was initialized with
        pub face_model_version: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::CollectionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CollectionArgs,
    ) -> CollectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collection_id_binding = args.collection_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rekognition/collection:Collection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectionId".into(),
                    value: collection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CollectionResult {
            arn: o.get_field("arn"),
            collection_id: o.get_field("collectionId"),
            face_model_version: o.get_field("faceModelVersion"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
