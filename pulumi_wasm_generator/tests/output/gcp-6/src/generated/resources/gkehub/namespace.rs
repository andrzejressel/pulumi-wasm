/// Namespace represents a namespace across the Fleet.
///
///
/// To get more information about Namespace, see:
///
/// * [API documentation](https://cloud.google.com/anthos/fleet-management/docs/reference/rest/v1/projects.locations.scopes.namespaces)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Namespace Basic
///
///
/// ```yaml
/// resources:
///   scope:
///     type: gcp:gkehub:Scope
///     properties:
///       scopeId: tf-test-scope_34535
///   namespace:
///     type: gcp:gkehub:Namespace
///     properties:
///       scopeNamespaceId: tf-test-namespace_22375
///       scopeId: ${scope.scopeId}
///       scope: ${scope.name}
///       namespaceLabels:
///         keyb: valueb
///         keya: valuea
///         keyc: valuec
///       labels:
///         keyb: valueb
///         keya: valuea
///         keyc: valuec
///     options:
///       dependsOn:
///         - ${scope}
/// ```
///
/// ## Import
///
/// Namespace can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/scopes/{{scope_id}}/namespaces/{{scope_namespace_id}}`
///
/// * `{{project}}/{{scope_id}}/{{scope_namespace_id}}`
///
/// * `{{scope_id}}/{{scope_namespace_id}}`
///
/// When using the `pulumi import` command, Namespace can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/namespace:Namespace default projects/{{project}}/locations/global/scopes/{{scope_id}}/namespaces/{{scope_namespace_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/namespace:Namespace default {{project}}/{{scope_id}}/{{scope_namespace_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/namespace:Namespace default {{scope_id}}/{{scope_namespace_id}}
/// ```
///
pub mod namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Labels for this Namespace.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Namespace-level cluster namespace labels. These labels are applied
        /// to the related namespace of the member clusters bound to the parent
        /// Scope. Scope-level labels (`namespace_labels` in the Fleet Scope
        /// resource) take precedence over Namespace-level labels if they share
        /// a key. Keys and values must be Kubernetes-conformant.
        #[builder(into, default)]
        pub namespace_labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Scope instance.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
        /// Id of the scope
        ///
        ///
        /// - - -
        #[builder(into)]
        pub scope_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The client-provided identifier of the namespace.
        #[builder(into)]
        pub scope_namespace_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Time the Namespace was deleted in UTC.
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels for this Namespace.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the namespace
        pub name: pulumi_wasm_rust::Output<String>,
        /// Namespace-level cluster namespace labels. These labels are applied
        /// to the related namespace of the member clusters bound to the parent
        /// Scope. Scope-level labels (`namespace_labels` in the Fleet Scope
        /// resource) take precedence over Namespace-level labels if they share
        /// a key. Keys and values must be Kubernetes-conformant.
        pub namespace_labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Scope instance.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// Id of the scope
        ///
        ///
        /// - - -
        pub scope_id: pulumi_wasm_rust::Output<String>,
        /// The client-provided identifier of the namespace.
        pub scope_namespace_id: pulumi_wasm_rust::Output<String>,
        /// State of the namespace resource.
        /// Structure is documented below.
        pub states: pulumi_wasm_rust::Output<
            Vec<super::super::types::gkehub::NamespaceState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let labels_binding = args.labels.get_output(context).get_inner();
        let namespace_labels_binding = args
            .namespace_labels
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let scope_id_binding = args.scope_id.get_output(context).get_inner();
        let scope_namespace_id_binding = args
            .scope_namespace_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:gkehub/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "namespaceLabels".into(),
                    value: &namespace_labels_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding,
                },
                register_interface::ObjectField {
                    name: "scopeNamespaceId".into(),
                    value: &scope_namespace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
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
                    name: "namespaceLabels".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "scopeId".into(),
                },
                register_interface::ResultField {
                    name: "scopeNamespaceId".into(),
                },
                register_interface::ResultField {
                    name: "states".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NamespaceResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
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
            namespace_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespaceLabels").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            scope_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeId").unwrap(),
            ),
            scope_namespace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scopeNamespaceId").unwrap(),
            ),
            states: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("states").unwrap(),
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
