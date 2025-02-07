pub mod get_assets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAssetsArgs {
        /// Outpost ARN.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Filters by list of Host IDs of a Dedicated Host.
        #[builder(into, default)]
        pub host_id_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Filters by list of state status. Valid values: "ACTIVE", "RETIRING".
        #[builder(into, default)]
        pub status_id_filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetAssetsResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of all the asset ids found. This data source will fail if none are found.
        pub asset_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub host_id_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub status_id_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAssetsArgs,
    ) -> GetAssetsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let host_id_filters_binding = args
            .host_id_filters
            .get_output(context)
            .get_inner();
        let status_id_filters_binding = args
            .status_id_filters
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:outposts/getAssets:getAssets".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "hostIdFilters".into(),
                    value: &host_id_filters_binding,
                },
                register_interface::ObjectField {
                    name: "statusIdFilters".into(),
                    value: &status_id_filters_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAssetsResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            asset_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assetIds"),
            ),
            host_id_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostIdFilters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            status_id_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusIdFilters"),
            ),
        }
    }
}
