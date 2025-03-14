/// Scope represents a Scope in a Fleet.
///
///
/// To get more information about Scope, see:
///
/// * [API documentation](https://cloud.google.com/anthos/fleet-management/docs/reference/rest/v1/projects.locations.scopes)
/// * How-to Guides
///     * [Registering a Cluster](https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster#register_cluster)
///
/// ## Example Usage
///
/// ### Gkehub Scope Basic
///
///
/// ```yaml
/// resources:
///   scope:
///     type: gcp:gkehub:Scope
///     properties:
///       scopeId: my-scope
///       namespaceLabels:
///         keyb: valueb
///         keya: valuea
///         keyc: valuec
///       labels:
///         keyb: valueb
///         keya: valuea
///         keyc: valuec
/// ```
///
/// ## Import
///
/// Scope can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/scopes/{{scope_id}}`
///
/// * `{{project}}/{{scope_id}}`
///
/// * `{{scope_id}}`
///
/// When using the `pulumi import` command, Scope can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/scope:Scope default projects/{{project}}/locations/global/scopes/{{scope_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/scope:Scope default {{project}}/{{scope_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/scope:Scope default {{scope_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScopeArgs {
        /// Labels for this Scope.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Scope-level cluster namespace labels. For the member clusters bound
        /// to the Scope, these labels are applied to each namespace under the
        /// Scope. Scope-level labels take precedence over Namespace-level
        /// labels (`namespace_labels` in the Fleet Namespace resource) if they
        /// share a key. Keys and values must be Kubernetes-conformant.
        #[builder(into, default)]
        pub namespace_labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client-provided identifier of the scope.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub scope_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ScopeResult {
        /// Time the Scope was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Time the Scope was deleted in UTC.
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Labels for this Scope.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique identifier of the scope
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Scope-level cluster namespace labels. For the member clusters bound
        /// to the Scope, these labels are applied to each namespace under the
        /// Scope. Scope-level labels take precedence over Namespace-level
        /// labels (`namespace_labels` in the Fleet Namespace resource) if they
        /// share a key. Keys and values must be Kubernetes-conformant.
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
        /// The client-provided identifier of the scope.
        ///
        ///
        /// - - -
        pub scope_id: pulumi_gestalt_rust::Output<String>,
        /// State of the scope resource.
        /// Structure is documented below.
        pub states: pulumi_gestalt_rust::Output<
            Vec<super::super::types::gkehub::ScopeState>,
        >,
        /// Google-generated UUID for this resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the Scope was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScopeArgs,
    ) -> ScopeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let labels_binding = args.labels.get_output(context);
        let namespace_labels_binding = args.namespace_labels.get_output(context);
        let project_binding = args.project.get_output(context);
        let scope_id_binding = args.scope_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/scope:Scope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespaceLabels".into(),
                    value: &namespace_labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scopeId".into(),
                    value: &scope_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScopeResult {
            create_time: o.get_field("createTime"),
            delete_time: o.get_field("deleteTime"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            namespace_labels: o.get_field("namespaceLabels"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            scope_id: o.get_field("scopeId"),
            states: o.get_field("states"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
