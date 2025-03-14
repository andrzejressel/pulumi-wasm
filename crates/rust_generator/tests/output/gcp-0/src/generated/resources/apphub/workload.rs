/// Workload represents a binary deployment (such as Managed Instance Groups (MIGs), GKE deployments, etc.) that performs the smallest logical subset of business functionality. It registers identified workload to the Application.
///
///
///
/// ## Example Usage
///
/// ## Import
///
/// Workload can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/applications/{{application_id}}/workloads/{{workload_id}}`
///
/// * `{{project}}/{{location}}/{{application_id}}/{{workload_id}}`
///
/// * `{{location}}/{{application_id}}/{{workload_id}}`
///
/// When using the `pulumi import` command, Workload can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default projects/{{project}}/locations/{{location}}/applications/{{application_id}}/workloads/{{workload_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default {{project}}/{{location}}/{{application_id}}/{{workload_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apphub/workload:Workload default {{location}}/{{application_id}}/{{workload_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workload {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkloadArgs {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apphub::WorkloadAttributes>,
        >,
        /// User-defined description of a Workload.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. The resource name of the original discovered workload.
        #[builder(into)]
        pub discovered_workload: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined name for the Workload.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Workload identifier.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workload_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkloadResult {
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Consumer provided attributes.
        /// Structure is documented below.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::apphub::WorkloadAttributes>,
        >,
        /// Output only. Create time.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-defined description of a Workload.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Immutable. The resource name of the original discovered workload.
        pub discovered_workload: pulumi_gestalt_rust::Output<String>,
        /// User-defined name for the Workload.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Part of `parent`.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The resource name of the Workload. Format:"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Output only. Workload state. Possible values:  STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. A universally unique identifier (UUID) for the `Workload` in the UUID4 format.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Update time.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// The Workload identifier.
        ///
        ///
        /// - - -
        pub workload_id: pulumi_gestalt_rust::Output<String>,
        /// Properties of an underlying compute resource represented by the Workload.
        /// Structure is documented below.
        pub workload_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apphub::WorkloadWorkloadProperty>,
        >,
        /// Reference of an underlying compute resource represented by the Workload.
        /// Structure is documented below.
        pub workload_references: pulumi_gestalt_rust::Output<
            Vec<super::super::types::apphub::WorkloadWorkloadReference>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkloadArgs,
    ) -> WorkloadResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let attributes_binding = args.attributes.get_output(context);
        let description_binding = args.description.get_output(context);
        let discovered_workload_binding = args.discovered_workload.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let workload_id_binding = args.workload_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apphub/workload:Workload".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "discoveredWorkload".into(),
                    value: &discovered_workload_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadId".into(),
                    value: &workload_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkloadResult {
            application_id: o.get_field("applicationId"),
            attributes: o.get_field("attributes"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            discovered_workload: o.get_field("discoveredWorkload"),
            display_name: o.get_field("displayName"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            workload_id: o.get_field("workloadId"),
            workload_properties: o.get_field("workloadProperties"),
            workload_references: o.get_field("workloadReferences"),
        }
    }
}
