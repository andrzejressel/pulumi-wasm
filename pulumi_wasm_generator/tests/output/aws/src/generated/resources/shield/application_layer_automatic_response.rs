/// Resource for managing an AWS Shield Application Layer Automatic Response for automatic DDoS mitigation.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_region::invoke(GetRegionArgs::builder().build_struct());
///     let currentGetCallerIdentity = get_caller_identity::invoke(
///         GetCallerIdentityArgs::builder().build_struct(),
///     );
///     let currentGetPartition = get_partition::invoke(
///         GetPartitionArgs::builder().build_struct(),
///     );
///     let example = application_layer_automatic_response::create(
///         "example",
///         ApplicationLayerAutomaticResponseArgs::builder()
///             .action("COUNT")
///             .resource_arn(
///                 "arn:${currentGetPartition.partition}:cloudfront:${currentGetCallerIdentity.accountId}:distribution/${distributionId}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod application_layer_automatic_response {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseArgs {
        /// One of `COUNT` or `BLOCK`
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::shield::ApplicationLayerAutomaticResponseTimeouts,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationLayerAutomaticResponseResult {
        /// One of `COUNT` or `BLOCK`
        pub action: pulumi_wasm_rust::Output<String>,
        /// ARN of the resource to protect (Cloudfront Distributions and ALBs only at this time).
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<
                super::super::types::shield::ApplicationLayerAutomaticResponseTimeouts,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApplicationLayerAutomaticResponseArgs,
    ) -> ApplicationLayerAutomaticResponseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/applicationLayerAutomaticResponse:ApplicationLayerAutomaticResponse"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationLayerAutomaticResponseResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
