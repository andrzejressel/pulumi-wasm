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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// Optional. Description of the zone.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Specification of the discovery feature applied to data in this zone.
        #[builder(into)]
        pub discovery_spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataplex::ZoneDiscoverySpec,
        >,
        /// Optional. User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. User defined labels for the zone. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake for the resource
        #[builder(into)]
        pub lake: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the zone.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Immutable. Specification of the resources that are referenced by the assets within this zone.
        #[builder(into)]
        pub resource_spec: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataplex::ZoneResourceSpec,
        >,
        /// Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// Output only. Aggregated status of the underlying assets of the zone.
        pub asset_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataplex::ZoneAssetStatus>,
        >,
        /// Output only. The time when the zone was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. Description of the zone.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. Specification of the discovery feature applied to data in this zone.
        pub discovery_spec: pulumi_gestalt_rust::Output<
            super::super::types::dataplex::ZoneDiscoverySpec,
        >,
        /// Optional. User friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. User defined labels for the zone. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field `effective_labels` for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake for the resource
        pub lake: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the zone.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. Immutable. Specification of the resources that are referenced by the assets within this zone.
        pub resource_spec: pulumi_gestalt_rust::Output<
            super::super::types::dataplex::ZoneResourceSpec,
        >,
        /// Output only. Current state of the zone. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. The type of the zone. Possible values: TYPE_UNSPECIFIED, RAW, CURATED
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Output only. System generated globally unique ID for the zone. This ID will be different if the zone is deleted and re-created with the same name.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when the zone was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneArgs,
    ) -> ZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let discovery_spec_binding = args.discovery_spec.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let lake_binding = args.lake.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let resource_spec_binding = args.resource_spec.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/zone:Zone".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "discoverySpec".into(),
                    value: discovery_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lake".into(),
                    value: lake_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSpec".into(),
                    value: resource_spec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneResult {
            asset_statuses: o.get_field("assetStatuses"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            discovery_spec: o.get_field("discoverySpec"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            lake: o.get_field("lake"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            resource_spec: o.get_field("resourceSpec"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
