#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_addon_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAddonVersionArgs {
        /// Name of the EKS add-on. The name must match one of
        /// the names returned by [list-addon](https://docs.aws.amazon.com/cli/latest/reference/eks/list-addons.html).
        #[builder(into)]
        pub addon_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of the EKS Cluster. Must be between 1-100 characters in length. Must begin with an alphanumeric character, and must only contain alphanumeric characters, dashes and underscores (`^[0-9A-Za-z][A-Za-z0-9\-_]+$`).
        #[builder(into)]
        pub kubernetes_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines if the most recent or default version of the addon should be returned.
        #[builder(into, default)]
        pub most_recent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetAddonVersionResult {
        pub addon_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kubernetes_version: pulumi_gestalt_rust::Output<String>,
        pub most_recent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Version of the EKS add-on.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAddonVersionArgs,
    ) -> GetAddonVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let addon_name_binding = args.addon_name.get_output(context);
        let kubernetes_version_binding = args.kubernetes_version.get_output(context);
        let most_recent_binding = args.most_recent.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:eks/getAddonVersion:getAddonVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addonName".into(),
                    value: addon_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesVersion".into(),
                    value: kubernetes_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mostRecent".into(),
                    value: most_recent_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAddonVersionResult {
            addon_name: o.get_field("addonName"),
            id: o.get_field("id"),
            kubernetes_version: o.get_field("kubernetesVersion"),
            most_recent: o.get_field("mostRecent"),
            version: o.get_field("version"),
        }
    }
}
