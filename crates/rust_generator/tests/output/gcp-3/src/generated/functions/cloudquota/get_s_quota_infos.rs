#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_s_quota_infos {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSQuotaInfosArgs {
        /// Parent value of QuotaInfo resources. Listing across different resource containers (such as 'projects/-') is not allowed. Allowed parents are "projects/[project-id / number]" or "folders/[folder-id / number]" or "organizations/[org-id / number].
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the service in which the quotas are defined.
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSQuotaInfosResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// (Output) The list of QuotaInfo.
        pub quota_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudquota::GetSQuotaInfosQuotaInfo>,
        >,
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSQuotaInfosArgs,
    ) -> GetSQuotaInfosResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let parent_binding = args.parent.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudquota/getSQuotaInfos:getSQuotaInfos".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "service".into(),
                    value: service_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSQuotaInfosResult {
            id: o.get_field("id"),
            parent: o.get_field("parent"),
            quota_infos: o.get_field("quotaInfos"),
            service: o.get_field("service"),
        }
    }
}
