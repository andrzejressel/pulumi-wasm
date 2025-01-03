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
pub mod lake {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LakeArgs {
        /// Optional. Description of the lake.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User-defined labels for the lake.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Optional. Settings to manage lake and Dataproc Metastore service instance association.
        #[builder(into, default)]
        pub metastore: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::LakeMetastore>,
        >,
        /// The name of the lake.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LakeResult {
        /// Output only. Aggregated status of the underlying assets of the lake.
        pub asset_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::LakeAssetStatus>,
        >,
        /// Output only. The time when the lake was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. Description of the lake.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. User-defined labels for the lake.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Optional. Settings to manage lake and Dataproc Metastore service instance association.
        pub metastore: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::LakeMetastore>,
        >,
        /// Output only. Metastore status of the lake.
        pub metastore_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::LakeMetastoreStatus>,
        >,
        /// The name of the lake.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Service account associated with this lake. This service account must be authorized to access or operate on resources managed by the lake.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// Output only. Current state of the lake. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. System generated globally unique ID for the lake. This ID will be different if the lake is deleted and re-created with the same name.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when the lake was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LakeArgs) -> LakeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let metastore_binding = args.metastore.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/lake:Lake".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "metastore".into(),
                    value: &metastore_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
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
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metastore".into(),
                },
                register_interface::ResultField {
                    name: "metastoreStatuses".into(),
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
                    name: "serviceAccount".into(),
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
        LakeResult {
            asset_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assetStatuses").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
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
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metastore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metastore").unwrap(),
            ),
            metastore_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metastoreStatuses").unwrap(),
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
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
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
