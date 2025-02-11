#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The location of the instance. eg us-central1
        ///
        /// - - -
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud Run v2 Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub binary_authorizations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceBinaryAuthorization>,
        >,
        pub client: pulumi_gestalt_rust::Output<String>,
        pub client_version: pulumi_gestalt_rust::Output<String>,
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceCondition>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub creator: pulumi_gestalt_rust::Output<String>,
        pub custom_audiences: pulumi_gestalt_rust::Output<Vec<String>>,
        pub default_uri_disabled: pulumi_gestalt_rust::Output<bool>,
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        pub generation: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ingress: pulumi_gestalt_rust::Output<String>,
        pub invoker_iam_disabled: pulumi_gestalt_rust::Output<bool>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub last_modifier: pulumi_gestalt_rust::Output<String>,
        pub latest_created_revision: pulumi_gestalt_rust::Output<String>,
        pub latest_ready_revision: pulumi_gestalt_rust::Output<String>,
        pub launch_stage: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub observed_generation: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        pub scalings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceScaling>,
        >,
        pub templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTemplate>,
        >,
        pub terminal_conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTerminalCondition>,
        >,
        pub traffic_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTrafficStatus>,
        >,
        pub traffics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrunv2::GetServiceTraffic>,
        >,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub uri: pulumi_gestalt_rust::Output<String>,
        pub urls: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudrunv2/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            annotations: o.get_field("annotations"),
            binary_authorizations: o.get_field("binaryAuthorizations"),
            client: o.get_field("client"),
            client_version: o.get_field("clientVersion"),
            conditions: o.get_field("conditions"),
            create_time: o.get_field("createTime"),
            creator: o.get_field("creator"),
            custom_audiences: o.get_field("customAudiences"),
            default_uri_disabled: o.get_field("defaultUriDisabled"),
            delete_time: o.get_field("deleteTime"),
            deletion_protection: o.get_field("deletionProtection"),
            description: o.get_field("description"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            expire_time: o.get_field("expireTime"),
            generation: o.get_field("generation"),
            id: o.get_field("id"),
            ingress: o.get_field("ingress"),
            invoker_iam_disabled: o.get_field("invokerIamDisabled"),
            labels: o.get_field("labels"),
            last_modifier: o.get_field("lastModifier"),
            latest_created_revision: o.get_field("latestCreatedRevision"),
            latest_ready_revision: o.get_field("latestReadyRevision"),
            launch_stage: o.get_field("launchStage"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            observed_generation: o.get_field("observedGeneration"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            scalings: o.get_field("scalings"),
            templates: o.get_field("templates"),
            terminal_conditions: o.get_field("terminalConditions"),
            traffic_statuses: o.get_field("trafficStatuses"),
            traffics: o.get_field("traffics"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            uri: o.get_field("uri"),
            urls: o.get_field("urls"),
        }
    }
}
