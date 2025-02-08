/// ## Example Usage
///
/// ### Network Services Mesh Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:Mesh
///     properties:
///       name: my-mesh
///       labels:
///         foo: bar
///       description: my description
///       interceptionPort: 443
/// ```
/// ### Network Services Mesh No Port
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networkservices:Mesh
///     properties:
///       name: my-mesh-noport
///       labels:
///         foo: bar
///       description: my description
/// ```
///
/// ## Import
///
/// Mesh can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/meshes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Mesh can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/mesh:Mesh default projects/{{project}}/locations/global/meshes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/mesh:Mesh default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/mesh:Mesh default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mesh {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MeshArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the
        /// specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to
        /// be redirected to this port regardless of its actual ip:port destination. If unset, a port
        /// '15001' is used as the interception port. This will is applicable only for sidecar proxy
        /// deployments.
        #[builder(into, default)]
        pub interception_port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Set of label tags associated with the Mesh resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Short name of the Mesh resource to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MeshResult {
        /// Time the Mesh was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the
        /// specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to
        /// be redirected to this port regardless of its actual ip:port destination. If unset, a port
        /// '15001' is used as the interception port. This will is applicable only for sidecar proxy
        /// deployments.
        pub interception_port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Set of label tags associated with the Mesh resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Short name of the Mesh resource to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Time the Mesh was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MeshArgs,
    ) -> MeshResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let interception_port_binding = args
            .interception_port
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/mesh:Mesh".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "interceptionPort".into(),
                    value: &interception_port_binding,
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
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MeshResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            interception_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("interceptionPort"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
