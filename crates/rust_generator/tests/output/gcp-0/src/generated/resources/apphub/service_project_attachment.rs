/// Represents a Service project attachment to the Host Project.
///
///
///
/// ## Example Usage
///
/// ### Service Project Attachment Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:apphub:ServiceProjectAttachment
///     properties:
///       serviceProjectAttachmentId: ${serviceProject.projectId}
///     options:
///       dependsOn:
///         - ${wait120s}
///   serviceProject:
///     type: gcp:organizations:Project
///     name: service_project
///     properties:
///       projectId: project-1
///       name: Service Project
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   wait120s:
///     type: time:sleep
///     name: wait_120s
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${serviceProject}
/// ```
/// ### Service Project Attachment Full
///
///
/// ```yaml
/// resources:
///   example2:
///     type: gcp:apphub:ServiceProjectAttachment
///     properties:
///       serviceProjectAttachmentId: ${serviceProjectFull.projectId}
///       serviceProject: ${serviceProjectFull.projectId}
///     options:
///       dependsOn:
///         - ${wait120s}
///   serviceProjectFull:
///     type: gcp:organizations:Project
///     name: service_project_full
///     properties:
///       projectId: project-1
///       name: Service Project Full
///       orgId: '123456789'
///       deletionPolicy: DELETE
///   wait120s:
///     type: time:sleep
///     name: wait_120s
///     properties:
///       createDuration: 120s
///     options:
///       dependsOn:
///         - ${serviceProjectFull}
/// ```
///
/// ## Import
///
/// ServiceProjectAttachment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/serviceProjectAttachments/{{service_project_attachment_id}}`
///
/// * `{{project}}/{{service_project_attachment_id}}`
///
/// * `{{service_project_attachment_id}}`
///
/// When using the `pulumi import` command, ServiceProjectAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apphub/serviceProjectAttachment:ServiceProjectAttachment default projects/{{project}}/locations/global/serviceProjectAttachments/{{service_project_attachment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/serviceProjectAttachment:ServiceProjectAttachment default {{project}}/{{service_project_attachment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/serviceProjectAttachment:ServiceProjectAttachment default {{service_project_attachment_id}}
/// ```
///
pub mod service_project_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceProjectAttachmentArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// "Immutable. Service project name in the format: \"projects/abc\"
        /// or \"projects/123\". As input, project name with either project id or number
        /// are accepted. As output, this field will contain project number."
        #[builder(into, default)]
        pub service_project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Required. The service project attachment identifier must contain the project_id of the service project specified in the service_project_attachment.service_project field. Hint: "projects/{project_id}"
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_project_attachment_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceProjectAttachmentResult {
        /// Output only. Create time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// "Identifier. The resource name of a ServiceProjectAttachment. Format:\"projects/{host-project-id}/locations/global/serviceProjectAttachments/{service-project-id}.\""
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// "Immutable. Service project name in the format: \"projects/abc\"
        /// or \"projects/123\". As input, project name with either project id or number
        /// are accepted. As output, this field will contain project number."
        pub service_project: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. The service project attachment identifier must contain the project_id of the service project specified in the service_project_attachment.service_project field. Hint: "projects/{project_id}"
        ///
        ///
        /// - - -
        pub service_project_attachment_id: pulumi_wasm_rust::Output<String>,
        /// ServiceProjectAttachment state.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A globally unique identifier (in UUID4 format) for the `ServiceProjectAttachment`.
        pub uid: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceProjectAttachmentArgs,
    ) -> ServiceProjectAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let service_project_binding = args
            .service_project
            .get_output(context)
            .get_inner();
        let service_project_attachment_id_binding = args
            .service_project_attachment_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apphub/serviceProjectAttachment:ServiceProjectAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceProject".into(),
                    value: &service_project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceProjectAttachmentId".into(),
                    value: &service_project_attachment_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceProjectAttachmentResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceProject"),
            ),
            service_project_attachment_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceProjectAttachmentId"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
        }
    }
}
