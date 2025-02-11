/// Resource for managing an AWS Location Geofence Collection.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = geofence_collection::create(
///         "example",
///         GeofenceCollectionArgs::builder().collection_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Location Geofence Collection using the `collection_name`. For example:
///
/// ```sh
/// $ pulumi import aws:location/geofenceCollection:GeofenceCollection example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod geofence_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GeofenceCollectionArgs {
        /// The name of the geofence collection.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub collection_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The optional description for the geofence collection.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value tags for the geofence collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GeofenceCollectionResult {
        /// The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all AWS.
        pub collection_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the geofence collection.
        ///
        /// The following arguments are optional:
        pub collection_name: pulumi_gestalt_rust::Output<String>,
        /// The timestamp for when the geofence collection resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The optional description for the geofence collection.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value tags for the geofence collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the geofence collection resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GeofenceCollectionArgs,
    ) -> GeofenceCollectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collection_name_binding = args.collection_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:location/geofenceCollection:GeofenceCollection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectionName".into(),
                    value: &collection_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GeofenceCollectionResult {
            collection_arn: o.get_field("collectionArn"),
            collection_name: o.get_field("collectionName"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            kms_key_id: o.get_field("kmsKeyId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            update_time: o.get_field("updateTime"),
        }
    }
}
