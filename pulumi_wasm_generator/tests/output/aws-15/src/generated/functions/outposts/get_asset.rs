pub mod get_asset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAssetArgs {
        /// Outpost ARN.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the asset.
        #[builder(into)]
        pub asset_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAssetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub asset_id: pulumi_wasm_rust::Output<String>,
        /// Type of the asset.
        pub asset_type: pulumi_wasm_rust::Output<String>,
        /// Host ID of the Dedicated Hosts on the asset, if a Dedicated Host is provisioned.
        pub host_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Position of an asset in a rack measured in rack units.
        pub rack_elevation: pulumi_wasm_rust::Output<i32>,
        /// Rack ID of the asset.
        pub rack_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAssetArgs,
    ) -> GetAssetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let asset_id_binding = args.asset_id.get_output(context).get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "assetId".into(),
                },
                register_interface::ResultField {
                    name: "assetType".into(),
                },
                register_interface::ResultField {
                    name: "hostId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "rackElevation".into(),
                },
                register_interface::ResultField {
                    name: "rackId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAssetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            asset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assetId").unwrap(),
            ),
            asset_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("assetType").unwrap(),
            ),
            host_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            rack_elevation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rackElevation").unwrap(),
            ),
            rack_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rackId").unwrap(),
            ),
        }
    }
}
