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
pub mod collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CollectionArgs {
        /// The name of the collection
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub collection_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::rekognition::CollectionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct CollectionResult {
        /// ARN of the Collection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the collection
        ///
        /// The following arguments are optional:
        pub collection_id: pulumi_wasm_rust::Output<String>,
        /// The Face Model Version that the collection was initialized with
        pub face_model_version: pulumi_wasm_rust::Output<String>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::rekognition::CollectionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CollectionArgs,
    ) -> CollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let collection_id_binding = args.collection_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rekognition/collection:Collection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collectionId".into(),
                    value: &collection_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CollectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            collection_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("collectionId"),
            ),
            face_model_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("faceModelVersion"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
