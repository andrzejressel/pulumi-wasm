/// The Dataplex Zone resource
///
/// ## Example Usage
///
/// ### Basic_zone
/// A basic example of a dataplex zone
/// ```yaml
/// resources:
///   primary:
///     type: gcp:dataplex:Zone
///     properties:
///       discoverySpec:
///         enabled: false
///       lake: ${basic.name}
///       location: us-west1
///       name: zone
///       resourceSpec:
///         locationType: MULTI_REGION
///       type: RAW
///       description: Zone for DCL
///       displayName: Zone for DCL
///       project: my-project-name
///       labels: {}
///   basic:
///     type: gcp:dataplex:Lake
///     properties:
///       location: us-west1
///       name: lake
///       description: Lake for DCL
///       displayName: Lake for DCL
///       project: my-project-name
///       labels:
///         my-lake: exists
/// ```
///
/// ## Import
///
/// Zone can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{name}}`
///
/// * `{{project}}/{{location}}/{{lake}}/{{name}}`
///
/// * `{{location}}/{{lake}}/{{name}}`
///
/// When using the `pulumi import` command, Zone can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/zone:Zone default projects/{{project}}/locations/{{location}}/lakes/{{lake}}/zones/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/zone:Zone default {{project}}/{{location}}/{{lake}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/zone:Zone default {{location}}/{{lake}}/{{name}}
/// ```
///
pub mod zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// Optional. Description of the zone.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Specification of the discovery feature applied to data in this zone.
        #[builder(into)]
        pub discovery_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::ZoneDiscoverySpec,
        >,
        /// Optional. User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User defined labels for the zone. **Note**: This field is non-authoritative, and will only manage the labels
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
        /// The name of the zone.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Immutable. Specification of the resources that are referenced by the assets within this zone.
        #[builder(into)]
        pub resource_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::ZoneResourceSpec,
        >,
        /// Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// Output only. Aggregated status of the underlying assets of the zone.
        pub asset_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::ZoneAssetStatus>,
        >,
        /// Output only. The time when the zone was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the zone.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Specification of the discovery feature applied to data in this zone.
        pub discovery_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::ZoneDiscoverySpec,
        >,
        /// Optional. User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. User defined labels for the zone. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake for the resource
        pub lake: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the zone.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. Immutable. Specification of the resources that are referenced by the assets within this zone.
        pub resource_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::ZoneResourceSpec,
        >,
        /// Output only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Output only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when the zone was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ZoneArgs) -> ZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let discovery_spec_binding = args.discovery_spec.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let lake_binding = args.lake.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let resource_spec_binding = args.resource_spec.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/zone:Zone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "assetStatuses".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "discoverySpec".into(),
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
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
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
        ZoneResult {
            asset_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assetStatuses").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            discovery_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("discoverySpec").unwrap(),
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
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
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
