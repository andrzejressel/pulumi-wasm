#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_asset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAssetArgs {
        /// Outpost ARN.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the asset.
        #[builder(into)]
        pub asset_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAssetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub asset_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the asset.
        pub asset_type: pulumi_gestalt_rust::Output<String>,
        /// Host ID of the Dedicated Hosts on the asset, if a Dedicated Host is provisioned.
        pub host_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Position of an asset in a rack measured in rack units.
        pub rack_elevation: pulumi_gestalt_rust::Output<i32>,
        /// Rack ID of the asset.
        pub rack_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAssetArgs,
    ) -> GetAssetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let asset_id_binding = args.asset_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:outposts/getAsset:getAsset".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assetId".into(),
                    value: &asset_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAssetResult {
            arn: o.get_field("arn"),
            asset_id: o.get_field("assetId"),
            asset_type: o.get_field("assetType"),
            host_id: o.get_field("hostId"),
            id: o.get_field("id"),
            rack_elevation: o.get_field("rackElevation"),
            rack_id: o.get_field("rackId"),
        }
    }
}
