pub mod get_geofence_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGeofenceCollectionArgs {
        /// Name of the geofence collection.
        #[builder(into)]
        pub collection_name: pulumi_wasm_rust::Output<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the geofence collection.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetGeofenceCollectionResult {
        /// ARN for the geofence collection resource. Used when you need to specify a resource across all AWS.
        pub collection_arn: pulumi_wasm_rust::Output<String>,
        pub collection_name: pulumi_wasm_rust::Output<String>,
        /// Timestamp for when the geofence collection resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional description of the geofence collection resource.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the geofence collection.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Timestamp for when the geofence collection resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGeofenceCollectionArgs) -> GetGeofenceCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let collection_name_binding = args.collection_name.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:location/getGeofenceCollection:getGeofenceCollection".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collectionName".into(),
                    value: &collection_name_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "collectionArn".into(),
                },
                register_interface::ResultField {
                    name: "collectionName".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGeofenceCollectionResult {
            collection_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionArn").unwrap(),
            ),
            collection_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collectionName").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}