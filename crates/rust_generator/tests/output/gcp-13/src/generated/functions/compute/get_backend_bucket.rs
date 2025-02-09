#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_backend_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackendBucketArgs {
        /// Name of the resource.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackendBucketResult {
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        pub cdn_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetBackendBucketCdnPolicy>,
        >,
        pub compression_mode: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub custom_response_headers: pulumi_gestalt_rust::Output<Vec<String>>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub edge_security_policy: pulumi_gestalt_rust::Output<String>,
        pub enable_cdn: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBackendBucketArgs,
    ) -> GetBackendBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getBackendBucket:getBackendBucket".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBackendBucketResult {
            bucket_name: o.get_field("bucketName"),
            cdn_policies: o.get_field("cdnPolicies"),
            compression_mode: o.get_field("compressionMode"),
            creation_timestamp: o.get_field("creationTimestamp"),
            custom_response_headers: o.get_field("customResponseHeaders"),
            description: o.get_field("description"),
            edge_security_policy: o.get_field("edgeSecurityPolicy"),
            enable_cdn: o.get_field("enableCdn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
        }
    }
}
