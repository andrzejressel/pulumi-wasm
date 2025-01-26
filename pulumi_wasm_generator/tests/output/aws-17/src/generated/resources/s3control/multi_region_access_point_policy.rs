/// Provides a resource to manage an S3 Multi-Region Access Point access control policy.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Multi-Region Access Point Policies using the `account_id` and `name` of the Multi-Region Access Point separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy example 123456789012:example
/// ```
pub mod multi_region_access_point_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyArgs {
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        #[builder(into)]
        pub details: pulumi_wasm_rust::InputOrOutput<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyResult {
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        pub details: pulumi_wasm_rust::Output<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
        /// The last established policy for the Multi-Region Access Point.
        pub established: pulumi_wasm_rust::Output<String>,
        /// The proposed policy for the Multi-Region Access Point.
        pub proposed: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MultiRegionAccessPointPolicyArgs,
    ) -> MultiRegionAccessPointPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let details_binding = args.details.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "details".into(),
                    value: &details_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "details".into(),
                },
                register_interface::ResultField {
                    name: "established".into(),
                },
                register_interface::ResultField {
                    name: "proposed".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MultiRegionAccessPointPolicyResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("details").unwrap(),
            ),
            established: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("established").unwrap(),
            ),
            proposed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proposed").unwrap(),
            ),
        }
    }
}
