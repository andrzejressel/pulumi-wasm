/// Application is a functional grouping of Services and Workloads that helps achieve a desired end-to-end business functionality. Services and Workloads are owned by the Application.
///
///
///
/// ## Example Usage
///
/// ### Apphub Application Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:apphub:Application
///     properties:
///       location: us-east1
///       applicationId: example-application
///       scope:
///         type: REGIONAL
/// ```
/// ### Apphub Application Global Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:apphub:Application
///     properties:
///       location: global
///       applicationId: example-application
///       scope:
///         type: GLOBAL
/// ```
/// ### Apphub Application Full
///
///
/// ```yaml
/// resources:
///   example2:
///     type: gcp:apphub:Application
///     properties:
///       location: us-east1
///       applicationId: example-application
///       displayName: Application Full
///       scope:
///         type: REGIONAL
///       description: Application for testing
///       attributes:
///         environment:
///           type: STAGING
///         criticality:
///           type: MISSION_CRITICAL
///         businessOwners:
///           - displayName: Alice
///             email: alice@google.com
///         developerOwners:
///           - displayName: Bob
///             email: bob@google.com
///         operatorOwners:
///           - displayName: Charlie
///             email: charlie@google.com
/// ```
///
/// ## Import
///
/// Application can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/applications/{{application_id}}`
///
/// * `{{project}}/{{location}}/{{application_id}}`
///
/// * `{{location}}/{{application_id}}`
///
/// When using the `pulumi import` command, Application can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apphub/application:Application default projects/{{project}}/locations/{{location}}/applications/{{application_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/application:Application default {{project}}/{{location}}/{{application_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/application:Application default {{location}}/{{application_id}}
/// ```
///
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// Required. The Application identifier.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Consumer provided attributes.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::apphub::ApplicationAttributes>,
        >,
        /// Optional. User-defined description of an Application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. User-defined name for the Application.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Part of `parent`. See documentation of `projectsId`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Scope of an application.
        /// Structure is documented below.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<
            super::super::types::apphub::ApplicationScope,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Required. The Application identifier.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// Consumer provided attributes.
        pub attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::apphub::ApplicationAttributes>,
        >,
        /// Output only. Create time.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. User-defined description of an Application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. User-defined name for the Application.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Part of `parent`. See documentation of `projectsId`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The resource name of an Application. Format:
        /// "projects/{host-project-id}/locations/{location}/applications/{application-id}"
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Scope of an application.
        /// Structure is documented below.
        pub scope: pulumi_wasm_rust::Output<
            super::super::types::apphub::ApplicationScope,
        >,
        /// Output only. Application state.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// CREATING
        /// ACTIVE
        /// DELETING
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A universally unique identifier (in UUID4 format) for the `Application`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Update time.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apphub/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(o.extract_field("scope")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
