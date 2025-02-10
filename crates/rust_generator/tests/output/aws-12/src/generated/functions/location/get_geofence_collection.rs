#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_geofence_collection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGeofenceCollectionArgs {
        /// Name of the geofence collection.
        #[builder(into)]
        pub collection_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the geofence collection.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetGeofenceCollectionResult {
        /// ARN for the geofence collection resource. Used when you need to specify a resource across all AWS.
        pub collection_arn: pulumi_gestalt_rust::Output<String>,
        pub collection_name: pulumi_gestalt_rust::Output<String>,
        /// Timestamp for when the geofence collection resource was created in ISO 8601 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional description of the geofence collection resource.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the geofence collection.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the geofence collection resource was last updated in ISO 8601 format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGeofenceCollectionArgs,
    ) -> GetGeofenceCollectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collection_name_binding = args.collection_name.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:location/getGeofenceCollection:getGeofenceCollection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectionName".into(),
                    value: collection_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: kms_key_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGeofenceCollectionResult {
            collection_arn: o.get_field("collectionArn"),
            collection_name: o.get_field("collectionName"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            kms_key_id: o.get_field("kmsKeyId"),
            tags: o.get_field("tags"),
            update_time: o.get_field("updateTime"),
        }
    }
}
