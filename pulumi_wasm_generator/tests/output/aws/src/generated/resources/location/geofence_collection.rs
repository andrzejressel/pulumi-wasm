/// Resource for managing an AWS Location Geofence Collection.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod geofence_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GeofenceCollectionArgs {
        /// The name of the geofence collection.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub collection_name: pulumi_wasm_rust::Output<String>,
        /// The optional description for the geofence collection.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the geofence collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GeofenceCollectionResult {
        /// The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all AWS.
        pub collection_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the geofence collection.
        ///
        /// The following arguments are optional:
        pub collection_name: pulumi_wasm_rust::Output<String>,
        /// The timestamp for when the geofence collection resource was created in ISO 8601 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The optional description for the geofence collection.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A key identifier for an AWS KMS customer managed key assigned to the Amazon Location resource.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the geofence collection. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp for when the geofence collection resource was last updated in ISO 8601 format.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GeofenceCollectionArgs) -> GeofenceCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let collection_name_binding = args.collection_name.get_inner();
        let description_binding = args.description.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:location/geofenceCollection:GeofenceCollection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "collectionName".into(),
                    value: &collection_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GeofenceCollectionResult {
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
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
