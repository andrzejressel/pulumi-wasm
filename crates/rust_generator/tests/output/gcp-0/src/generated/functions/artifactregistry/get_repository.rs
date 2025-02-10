#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_repository {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRepositoryArgs {
        /// The location of the artifact registry repository. eg us-central1
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The last part of the repository name.
        #[builder(into)]
        pub repository_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRepositoryResult {
        pub cleanup_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryCleanupPolicy>,
        >,
        pub cleanup_policy_dry_run: pulumi_gestalt_rust::Output<bool>,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub docker_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryDockerConfig>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub maven_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::artifactregistry::GetRepositoryMavenConfig>,
        >,
        pub mode: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub remote_repository_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::artifactregistry::GetRepositoryRemoteRepositoryConfig,
            >,
        >,
        pub repository_id: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub virtual_repository_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::artifactregistry::GetRepositoryVirtualRepositoryConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let repository_id_binding = args.repository_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:artifactregistry/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "repositoryId".into(),
                    value: repository_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRepositoryResult {
            cleanup_policies: o.get_field("cleanupPolicies"),
            cleanup_policy_dry_run: o.get_field("cleanupPolicyDryRun"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            docker_configs: o.get_field("dockerConfigs"),
            effective_labels: o.get_field("effectiveLabels"),
            format: o.get_field("format"),
            id: o.get_field("id"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maven_configs: o.get_field("mavenConfigs"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            remote_repository_configs: o.get_field("remoteRepositoryConfigs"),
            repository_id: o.get_field("repositoryId"),
            update_time: o.get_field("updateTime"),
            virtual_repository_configs: o.get_field("virtualRepositoryConfigs"),
        }
    }
}
