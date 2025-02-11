#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_attached_install_manifest {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAttachedInstallManifestArgs {
        /// The name that will be used when creating the attached cluster resource.
        #[builder(into)]
        pub cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location to list versions for.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The platform version for the cluster. A list of valid values can be retrieved using the `gcp.container.getAttachedVersions` data source.
        #[builder(into)]
        pub platform_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the project to list available platform versions for. Should match the project the cluster will be deployed to.
        /// Defaults to the project that the provider is authenticated with.
        #[builder(into)]
        pub project: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAttachedInstallManifestResult {
        pub cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A string with the YAML manifest that needs to be applied to the cluster.
        pub manifest: pulumi_gestalt_rust::Output<String>,
        pub platform_version: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAttachedInstallManifestArgs,
    ) -> GetAttachedInstallManifestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_id_binding = args.cluster_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let platform_version_binding = args.platform_version.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:container/getAttachedInstallManifest:getAttachedInstallManifest"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformVersion".into(),
                    value: &platform_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAttachedInstallManifestResult {
            cluster_id: o.get_field("clusterId"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            manifest: o.get_field("manifest"),
            platform_version: o.get_field("platformVersion"),
            project: o.get_field("project"),
        }
    }
}
