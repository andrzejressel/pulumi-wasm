#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_addon {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAddonArgs {
        /// Name of the EKS add-on. The name must match one of
        /// the names returned by [list-addon](https://docs.aws.amazon.com/cli/latest/reference/eks/list-addons.html).
        #[builder(into)]
        pub addon_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the EKS Cluster.
        #[builder(into)]
        pub cluster_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetAddonResult {
        pub addon_name: pulumi_gestalt_rust::Output<String>,
        /// Version of EKS add-on.
        pub addon_version: pulumi_gestalt_rust::Output<String>,
        /// ARN of the EKS add-on.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub cluster_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration values for the addon with a single JSON string.
        pub configuration_values: pulumi_gestalt_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was updated.
        pub modified_at: pulumi_gestalt_rust::Output<String>,
        /// Pod identity association for the EKS add-on.
        pub pod_identity_associations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::eks::GetAddonPodIdentityAssociation>,
        >,
        /// ARN of IAM role used for EKS add-on. If value is empty -
        /// then add-on uses the IAM role assigned to the EKS Cluster node.
        pub service_account_role_arn: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAddonArgs,
    ) -> GetAddonResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addon_name_binding = args.addon_name.get_output(context);
        let cluster_name_binding = args.cluster_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:eks/getAddon:getAddon".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonName".into(),
                    value: addon_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterName".into(),
                    value: cluster_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAddonResult {
            addon_name: o.get_field("addonName"),
            addon_version: o.get_field("addonVersion"),
            arn: o.get_field("arn"),
            cluster_name: o.get_field("clusterName"),
            configuration_values: o.get_field("configurationValues"),
            created_at: o.get_field("createdAt"),
            id: o.get_field("id"),
            modified_at: o.get_field("modifiedAt"),
            pod_identity_associations: o.get_field("podIdentityAssociations"),
            service_account_role_arn: o.get_field("serviceAccountRoleArn"),
            tags: o.get_field("tags"),
        }
    }
}
