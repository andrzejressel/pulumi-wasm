#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceArgs {
        /// The name of the location of the instance. This
        /// can be a region for ENTERPRISE tier instances. If it is not provided,
        /// the provider region or zone is used.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a Filestore instance.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceResult {
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        pub deletion_protection_reason: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub file_shares: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::filestore::GetInstanceFileShare>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key_name: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub networks: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::filestore::GetInstanceNetwork>,
        >,
        pub performance_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::filestore::GetInstancePerformanceConfig>,
        >,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub protocol: pulumi_gestalt_rust::Output<String>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub tier: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceArgs,
    ) -> GetInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:filestore/getInstance:getInstance".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
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
        GetInstanceResult {
            create_time: o.get_field("createTime"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            deletion_protection_reason: o.get_field("deletionProtectionReason"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            file_shares: o.get_field("fileShares"),
            id: o.get_field("id"),
            kms_key_name: o.get_field("kmsKeyName"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            networks: o.get_field("networks"),
            performance_configs: o.get_field("performanceConfigs"),
            project: o.get_field("project"),
            protocol: o.get_field("protocol"),
            pulumi_labels: o.get_field("pulumiLabels"),
            tier: o.get_field("tier"),
            zone: o.get_field("zone"),
        }
    }
}
