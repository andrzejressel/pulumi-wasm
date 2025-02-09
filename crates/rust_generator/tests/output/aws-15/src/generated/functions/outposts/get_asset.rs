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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAssetArgs,
    ) -> GetAssetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let asset_id_binding_1 = args.asset_id.get_output(context);
        let asset_id_binding = asset_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getAsset:getAsset".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "assetId".into(),
                    value: &asset_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAssetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            asset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assetId"),
            ),
            asset_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assetType"),
            ),
            host_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            rack_elevation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rackElevation"),
            ),
            rack_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rackId"),
            ),
        }
    }
}
