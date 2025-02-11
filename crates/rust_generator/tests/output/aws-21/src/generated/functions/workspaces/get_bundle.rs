#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_bundle {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBundleArgs {
        /// ID of the bundle.
        #[builder(into, default)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the bundle. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Owner of the bundles. You have to leave it blank for own bundles. You cannot combine this parameter with `bundle_id`.
        #[builder(into, default)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBundleResult {
        /// The ID of the bundle.
        pub bundle_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The compute type. See supported fields below.
        pub compute_types: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleComputeType>,
        >,
        /// The description of the bundle.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the compute type.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The owner of the bundle.
        pub owner: pulumi_gestalt_rust::Output<Option<String>>,
        /// The root volume. See supported fields below.
        pub root_storages: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleRootStorage>,
        >,
        /// The user storage. See supported fields below.
        pub user_storages: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::workspaces::GetBundleUserStorage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBundleArgs,
    ) -> GetBundleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bundle_id_binding = args.bundle_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_binding = args.owner.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:workspaces/getBundle:getBundle".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owner".into(),
                    value: &owner_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBundleResult {
            bundle_id: o.get_field("bundleId"),
            compute_types: o.get_field("computeTypes"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            root_storages: o.get_field("rootStorages"),
            user_storages: o.get_field("userStorages"),
        }
    }
}
