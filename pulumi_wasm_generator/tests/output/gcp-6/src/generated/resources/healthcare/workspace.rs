/// A Data Mapper workspace is used to configure Data Mapper access, permissions and data sources for mapping clinical patient data to the FHIR standard.
///
///
/// To get more information about Workspace, see:
///
/// * [API documentation](https://cloud.google.com/healthcare-api/healthcare-data-engine/docs/reference/rest/v1/projects.locations.datasets.dataMapperWorkspaces)
/// * How-to Guides
///     * [Create and manage Data Mapper workspaces ](https://cloud.google.com/healthcare-api/healthcare-data-engine/docs/manage-workspaces)
///
/// ## Example Usage
///
/// ### Healthcare Workspace Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:healthcare:Workspace
///     properties:
///       name: example-dm-workspace
///       dataset: ${dataset.id}
///       settings:
///         dataProjectIds:
///           - example-data-source-project-id
///       labels:
///         label1: labelvalue1
///   dataset:
///     type: gcp:healthcare:Dataset
///     properties:
///       name: example-dataset
///       location: us-central1
/// ```
///
/// ## Import
///
/// Workspace can be imported using any of these accepted formats:
///
/// * `{{dataset}}/dataMapperWorkspaces/{{name}}`
///
/// When using the `pulumi import` command, Workspace can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:healthcare/workspace:Workspace default {{dataset}}/dataMapperWorkspaces/{{name}}
/// ```
///
pub mod workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user labels. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg",
        /// "count": "3" } **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the workspace, in the format 'projects/{projectId}/locations/{location}/datasets/{datasetId}/dataMapperWorkspaces/{workspaceId}'
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Settings associated with this workspace.
        /// Structure is documented below.
        #[builder(into)]
        pub settings: pulumi_wasm_rust::InputOrOutput<
            super::super::types::healthcare::WorkspaceSettings,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// Identifies the dataset addressed by this request. Must be in the format
        /// 'projects/{project}/locations/{location}/datasets/{dataset}'
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The user labels. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg",
        /// "count": "3" } **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the workspace, in the format 'projects/{projectId}/locations/{location}/datasets/{datasetId}/dataMapperWorkspaces/{workspaceId}'
        pub name: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Settings associated with this workspace.
        /// Structure is documented below.
        pub settings: pulumi_wasm_rust::Output<
            super::super::types::healthcare::WorkspaceSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_binding = args.dataset.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let settings_binding = args.settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:healthcare/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "settings".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceResult {
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("settings").unwrap(),
            ),
        }
    }
}
