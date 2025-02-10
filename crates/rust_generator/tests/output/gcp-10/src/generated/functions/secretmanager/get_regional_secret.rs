#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_regional_secret {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionalSecretArgs {
        /// The location of the regional secret. eg us-central1
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the regional secret.
        #[builder(into)]
        pub secret_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegionalSecretResult {
        pub annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub customer_managed_encryptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::secretmanager::GetRegionalSecretCustomerManagedEncryption,
            >,
        >,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub rotations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetRegionalSecretRotation>,
        >,
        pub secret_id: pulumi_gestalt_rust::Output<String>,
        pub topics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::secretmanager::GetRegionalSecretTopic>,
        >,
        pub ttl: pulumi_gestalt_rust::Output<String>,
        pub version_aliases: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub version_destroy_ttl: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionalSecretArgs,
    ) -> GetRegionalSecretResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let secret_id_binding = args.secret_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:secretmanager/getRegionalSecret:getRegionalSecret".into(),
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
                    name: "secretId".into(),
                    value: secret_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionalSecretResult {
            annotations: o.get_field("annotations"),
            create_time: o.get_field("createTime"),
            customer_managed_encryptions: o.get_field("customerManagedEncryptions"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            expire_time: o.get_field("expireTime"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rotations: o.get_field("rotations"),
            secret_id: o.get_field("secretId"),
            topics: o.get_field("topics"),
            ttl: o.get_field("ttl"),
            version_aliases: o.get_field("versionAliases"),
            version_destroy_ttl: o.get_field("versionDestroyTtl"),
        }
    }
}
