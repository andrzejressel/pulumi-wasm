/// The Dataplex Lake resource
///
/// ## Example Usage
///
/// ### Basic_lake
/// A basic example of a dataplex lake
/// ```yaml
/// resources:
///   primary:
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
/// Lake can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lakes/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Lake can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/lake:Lake default projects/{{project}}/locations/{{location}}/lakes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/lake:Lake default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/lake:Lake default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lake {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LakeArgs {
        /// Optional. Description of the lake.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. User-defined labels for the lake.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Settings to manage lake and Dataproc Metastore service instance association.
        #[builder(into, default)]
        pub metastore: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataplex::LakeMetastore>,
        >,
        /// The name of the lake.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LakeResult {
        /// Output only. Aggregated status of the underlying assets of the lake.
        pub asset_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataplex::LakeAssetStatus>,
        >,
        /// Output only. The time when the lake was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. Description of the lake.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. User friendly display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. User-defined labels for the lake.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Optional. Settings to manage lake and Dataproc Metastore service instance association.
        pub metastore: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataplex::LakeMetastore>,
        >,
        /// Output only. Metastore status of the lake.
        pub metastore_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataplex::LakeMetastoreStatus>,
        >,
        /// The name of the lake.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Service account associated with this lake. This service account must be authorized to access or operate on resources managed by the lake.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Output only. Current state of the lake. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. System generated globally unique ID for the lake. This ID will be different if the lake is deleted and re-created with the same name.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when the lake was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LakeArgs,
    ) -> LakeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let metastore_binding = args.metastore.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/lake:Lake".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
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
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metastore".into(),
                    value: metastore_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LakeResult {
            asset_statuses: o.get_field("assetStatuses"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            metastore: o.get_field("metastore"),
            metastore_statuses: o.get_field("metastoreStatuses"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            service_account: o.get_field("serviceAccount"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
