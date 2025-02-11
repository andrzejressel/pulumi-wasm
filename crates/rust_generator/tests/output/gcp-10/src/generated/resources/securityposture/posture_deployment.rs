/// Represents a deployment of a security posture on a resource. A posture contains user curated policy sets. A posture can
/// be deployed on a project or on a folder or on an organization. To deploy a posture we need to populate the posture's name
/// and its revision_id in the posture deployment configuration. Every update to a deployed posture generates a new revision_id.
/// Thus, the updated revision_id should be used in the respective posture deployment's configuration to deploy that posture
/// on a resource.
///
///
/// To get more information about PostureDeployment, see:
/// * How-to Guides
///     * [Create and deploy a posture](https://cloud.google.com/security-command-center/docs/how-to-use-security-posture)
///
/// ## Import
///
/// PostureDeployment can be imported using any of these accepted formats:
///
/// * `{{parent}}/locations/{{location}}/postureDeployments/{{posture_deployment_id}}`
///
/// When using the `pulumi import` command, PostureDeployment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securityposture/postureDeployment:PostureDeployment default {{parent}}/locations/{{location}}/postureDeployments/{{posture_deployment_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod posture_deployment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PostureDeploymentArgs {
        /// Description of the posture deployment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the resource, eg. global`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the posture deployment.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub posture_deployment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Relative name of the posture which needs to be deployed. It should be in the format:
        /// organizations/{organization_id}/locations/{location}/postures/{posture_id}
        #[builder(into)]
        pub posture_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Revision_id the posture which needs to be deployed.
        #[builder(into)]
        pub posture_revision_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource on which the posture should be deployed. This can be in one of the following formats:
        /// projects/{project_number},
        /// folders/{folder_number},
        /// organizations/{organization_id}
        #[builder(into)]
        pub target_resource: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PostureDeploymentResult {
        /// Time the posture deployment was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the posture deployment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// This is an output only optional field which will be filled in case when
        /// PostureDeployment state is UPDATE_FAILED or CREATE_FAILED or DELETE_FAILED.
        /// It denotes the desired posture to be deployed.
        pub desired_posture_id: pulumi_gestalt_rust::Output<String>,
        /// This is an output only optional field which will be filled in case when
        /// PostureDeployment state is UPDATE_FAILED or CREATE_FAILED or DELETE_FAILED.
        /// It denotes the desired posture revision_id to be deployed.
        pub desired_posture_revision_id: pulumi_gestalt_rust::Output<String>,
        /// For Resource freshness validation (https://google.aip.dev/154)
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// This is a output only optional field which will be filled in case where
        /// PostureDeployment enters a failure state like UPDATE_FAILED or
        /// CREATE_FAILED or DELETE_FAILED. It will have the failure message for posture deployment's
        /// CREATE/UPDATE/DELETE methods.
        pub failure_message: pulumi_gestalt_rust::Output<String>,
        /// The location of the resource, eg. global`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Name of the posture deployment instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the resource, an organization. Format should be `organizations/{organization_id}`.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// ID of the posture deployment.
        ///
        ///
        /// - - -
        pub posture_deployment_id: pulumi_gestalt_rust::Output<String>,
        /// Relative name of the posture which needs to be deployed. It should be in the format:
        /// organizations/{organization_id}/locations/{location}/postures/{posture_id}
        pub posture_id: pulumi_gestalt_rust::Output<String>,
        /// Revision_id the posture which needs to be deployed.
        pub posture_revision_id: pulumi_gestalt_rust::Output<String>,
        /// If set, there are currently changes in flight to the posture deployment.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// State of the posture deployment. A posture deployment can be in the following terminal states:
        /// ACTIVE, CREATE_FAILED, UPDATE_FAILED, DELETE_FAILED.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The resource on which the posture should be deployed. This can be in one of the following formats:
        /// projects/{project_number},
        /// folders/{folder_number},
        /// organizations/{organization_id}
        pub target_resource: pulumi_gestalt_rust::Output<String>,
        /// Time the posture deployment was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PostureDeploymentArgs,
    ) -> PostureDeploymentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let location_binding = args.location.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let posture_deployment_id_binding = args
            .posture_deployment_id
            .get_output(context);
        let posture_id_binding = args.posture_id.get_output(context);
        let posture_revision_id_binding = args.posture_revision_id.get_output(context);
        let target_resource_binding = args.target_resource.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securityposture/postureDeployment:PostureDeployment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postureDeploymentId".into(),
                    value: &posture_deployment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postureId".into(),
                    value: &posture_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postureRevisionId".into(),
                    value: &posture_revision_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetResource".into(),
                    value: &target_resource_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PostureDeploymentResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            desired_posture_id: o.get_field("desiredPostureId"),
            desired_posture_revision_id: o.get_field("desiredPostureRevisionId"),
            etag: o.get_field("etag"),
            failure_message: o.get_field("failureMessage"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            posture_deployment_id: o.get_field("postureDeploymentId"),
            posture_id: o.get_field("postureId"),
            posture_revision_id: o.get_field("postureRevisionId"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            target_resource: o.get_field("targetResource"),
            update_time: o.get_field("updateTime"),
        }
    }
}
