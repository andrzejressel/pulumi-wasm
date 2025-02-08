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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRepositoryArgs,
    ) -> GetRepositoryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repository_id_binding = args.repository_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:artifactregistry/getRepository:getRepository".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRepositoryResult {
            cleanup_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cleanupPolicies"),
            ),
            cleanup_policy_dry_run: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cleanupPolicyDryRun"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            docker_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dockerConfigs"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("format"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kms_key_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            maven_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mavenConfigs"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            remote_repository_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remoteRepositoryConfigs"),
            ),
            repository_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            virtual_repository_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("virtualRepositoryConfigs"),
            ),
        }
    }
}
