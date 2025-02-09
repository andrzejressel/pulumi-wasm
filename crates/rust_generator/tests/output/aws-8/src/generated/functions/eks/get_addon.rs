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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAddonArgs,
    ) -> GetAddonResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let addon_name_binding_1 = args.addon_name.get_output(context);
        let addon_name_binding = addon_name_binding_1.get_inner();
        let cluster_name_binding_1 = args.cluster_name.get_output(context);
        let cluster_name_binding = cluster_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:eks/getAddon:getAddon".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonName".into(),
                    value: &addon_name_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAddonResult {
            addon_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addonName"),
            ),
            addon_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addonVersion"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            cluster_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            configuration_values: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationValues"),
            ),
            created_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            modified_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modifiedAt"),
            ),
            pod_identity_associations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("podIdentityAssociations"),
            ),
            service_account_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceAccountRoleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
