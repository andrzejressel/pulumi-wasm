pub mod get_origin_access_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginAccessControlArgs {
        /// The identifier for the origin access control settings. For example: `E2T5VTFBZJ3BJB`.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginAccessControlResult {
        /// A description of the origin access control.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Current version of the origin access control's information. For example: `E2QWRUHAPOMQZL`.
        pub etag: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// A name to identify the origin access control.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of origin that this origin access control is for.
        pub origin_access_control_origin_type: pulumi_wasm_rust::Output<String>,
        /// Specifies which requests CloudFront signs.
        pub signing_behavior: pulumi_wasm_rust::Output<String>,
        /// The signing protocol of the origin access control, which determines how CloudFront signs (authenticates) requests.
        pub signing_protocol: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOriginAccessControlArgs,
    ) -> GetOriginAccessControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getOriginAccessControl:getOriginAccessControl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "originAccessControlOriginType".into(),
                },
                register_interface::ResultField {
                    name: "signingBehavior".into(),
                },
                register_interface::ResultField {
                    name: "signingProtocol".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOriginAccessControlResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            origin_access_control_origin_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originAccessControlOriginType").unwrap(),
            ),
            signing_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingBehavior").unwrap(),
            ),
            signing_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingProtocol").unwrap(),
            ),
        }
    }
}
