/// The Dataplex Asset resource
///
/// ## Example Usage
///
/// ### Basic_asset
/// ```yaml
/// resources:
///   basicBucket:
///     type: gcp:storage:Bucket
///     name: basic_bucket
///     properties:
///       name: bucket
///       location: us-west1
///       uniformBucketLevelAccess: true
///       project: my-project-name
///   basicLake:
///     type: gcp:dataplex:Lake
///     name: basic_lake
///     properties:
///       name: lake
///       location: us-west1
///       project: my-project-name
///   basicZone:
///     type: gcp:dataplex:Zone
///     name: basic_zone
///     properties:
///       name: zone
///       location: us-west1
///       lake: ${basicLake.name}
///       type: RAW
///       discoverySpec:
///         enabled: false
///       resourceSpec:
///         locationType: SINGLE_REGION
///       project: my-project-name
///   primary:
///     type: gcp:dataplex:Asset
///     properties:
///       name: asset
///       location: us-west1
///       lake: ${basicLake.name}
///       dataplexZone: ${basicZone.name}
///       discoverySpec:
///         enabled: false
///       resourceSpec:
///         name: projects/my-project-name/buckets/bucket
///         type: STORAGE_BUCKET
///       labels:
///         env: foo
///         my-asset: exists
///       project: my-project-name
///     options:
///       dependsOn:
///         - ${basicBucket}
/// ```
///
/// ## Import
///
/// Asset can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{name}}`
///
/// * `{{project}}/{{location}}/{{lake}}/{{dataplex_zone}}/{{name}}`
///
/// * `{{location}}/{{lake}}/{{dataplex_zone}}/{{name}}`
///
/// When using the `pulumi import` command, Asset can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/asset:Asset default projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{dataplex_zone}}/assets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/asset:Asset default {{project}}/{{location}}/{{lake}}/{{dataplex_zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/asset:Asset default {{location}}/{{lake}}/{{dataplex_zone}}/{{name}}
/// ```
///
pub mod asset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssetArgs {
        /// The zone for the resource
        #[builder(into)]
        pub dataplex_zone: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the asset.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Specification of the discovery feature applied to data referenced by this asset. When this spec is left unset, the asset will use the spec set on the parent zone.
        #[builder(into)]
        pub discovery_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::AssetDiscoverySpec,
        >,
        /// Optional. User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User defined labels for the asset. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake for the resource
        #[builder(into)]
        pub lake: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the asset.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Immutable. Specification of the resource that is referenced by this asset.
        #[builder(into)]
        pub resource_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::AssetResourceSpec,
        >,
    }
    #[allow(dead_code)]
    pub struct AssetResult {
        /// Output only. The time when the asset was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The zone for the resource
        pub dataplex_zone: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the asset.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Specification of the discovery feature applied to data referenced by this asset. When this spec is left unset, the asset will use the spec set on the parent zone.
        pub discovery_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::AssetDiscoverySpec,
        >,
        /// Output only. Status of the discovery feature applied to data referenced by this asset.
        pub discovery_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::AssetDiscoveryStatus>,
        >,
        /// Optional. User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. User defined labels for the asset. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake for the resource
        pub lake: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the asset.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. Immutable. Specification of the resource that is referenced by this asset.
        pub resource_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::AssetResourceSpec,
        >,
        /// Output only. Status of the resource referenced by this asset.
        pub resource_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::AssetResourceStatus>,
        >,
        /// Output only. Status of the security policy applied to resource referenced by this asset.
        pub security_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::AssetSecurityStatus>,
        >,
        /// Output only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. System generated globally unique ID for the asset. This ID will be different if the asset is deleted and re-created with the same name.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when the asset was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AssetArgs) -> AssetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataplex_zone_binding = args.dataplex_zone.get_inner();
        let description_binding = args.description.get_inner();
        let discovery_spec_binding = args.discovery_spec.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let lake_binding = args.lake.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let resource_spec_binding = args.resource_spec.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/asset:Asset".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataplexZone".into(),
                    value: &dataplex_zone_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "discoverySpec".into(),
                    value: &discovery_spec_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "lake".into(),
                    value: &lake_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSpec".into(),
                    value: &resource_spec_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataplexZone".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "discoverySpec".into(),
                },
                register_interface::ResultField {
                    name: "discoveryStatuses".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lake".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "resourceSpec".into(),
                },
                register_interface::ResultField {
                    name: "resourceStatuses".into(),
                },
                register_interface::ResultField {
                    name: "securityStatuses".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
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
        AssetResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dataplex_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataplexZone").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            discovery_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoverySpec").unwrap(),
            ),
            discovery_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoveryStatuses").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            lake: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lake").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            resource_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSpec").unwrap(),
            ),
            resource_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceStatuses").unwrap(),
            ),
            security_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityStatuses").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
