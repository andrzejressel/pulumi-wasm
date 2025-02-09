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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Labels for this Namespace.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Namespace-level cluster namespace labels. These labels are applied
        /// to the related namespace of the member clusters bound to the parent
        /// Scope. Scope-level labels (`namespace_labels` in the Fleet Scope
        /// resource) take precedence over Namespace-level labels if they share
        /// a key. Keys and values must be Kubernetes-conformant.
        #[builder(into, default)]
        pub namespace_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Scope instance.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Id of the scope
        ///
        ///
        /// - - -
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The client-provided identifier of the namespace.
        #[builder(into)]
        pub scope_namespace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Time the Namespace was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Time the Namespace was deleted in UTC.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels for this Namespace.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the namespace
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Namespace-level cluster namespace labels. These labels are applied
        /// to the related namespace of the member clusters bound to the parent
        /// Scope. Scope-level labels (`namespace_labels` in the Fleet Scope
        /// resource) take precedence over Namespace-level labels if they share
        /// a key. Keys and values must be Kubernetes-conformant.
        pub namespace_labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Scope instance.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// Id of the scope
        ///
        ///
        /// - - -
        pub scope_id: pulumi_gestalt_rust::Output<String>,
        /// The client-provided identifier of the namespace.
        pub scope_namespace_id: pulumi_gestalt_rust::Output<String>,
        /// State of the namespace resource.
        /// Structure is documented below.
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkehub::NamespaceState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the Namespace was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let labels_binding = args.labels.get_output(context);
        let namespace_labels_binding = args.namespace_labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let scope_namespace_id_binding = args.scope_namespace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceLabels".into(),
                    value: namespace_labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: scope_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeNamespaceId".into(),
                    value: scope_namespace_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamespaceResult {
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            namespace_labels: o.get_field("namespaceLabels"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            scope: o.get_field("scope"),
            scope_id: o.get_field("scopeId"),
            scope_namespace_id: o.get_field("scopeNamespaceId"),
            states: o.get_field("states"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
