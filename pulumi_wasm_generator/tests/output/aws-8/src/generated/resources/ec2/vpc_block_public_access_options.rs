/// Resource for managing an AWS VPC Block Public Access Options.
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
///     let example = vpc_block_public_access_options::create(
///         "example",
///         VpcBlockPublicAccessOptionsArgs::builder()
///             .internet_gateway_block_mode("block-bidirectional")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Block Public Access Options using the `aws_region`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcBlockPublicAccessOptions:VpcBlockPublicAccessOptions example us-east-1
/// ```
pub mod vpc_block_public_access_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsArgs {
        /// Block mode. Needs to be one of `block-bidirectional`, `block-ingress`, `off`. If this resource is deleted, then this value will be set to `off` in the AWS account and region.
        #[builder(into)]
        pub internet_gateway_block_mode: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::VpcBlockPublicAccessOptionsTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcBlockPublicAccessOptionsResult {
        /// The AWS account id to which these options apply.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The AWS region to which these options apply.
        pub aws_region: pulumi_wasm_rust::Output<String>,
        /// Block mode. Needs to be one of `block-bidirectional`, `block-ingress`, `off`. If this resource is deleted, then this value will be set to `off` in the AWS account and region.
        pub internet_gateway_block_mode: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::VpcBlockPublicAccessOptionsTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VpcBlockPublicAccessOptionsArgs,
    ) -> VpcBlockPublicAccessOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let internet_gateway_block_mode_binding = args
            .internet_gateway_block_mode
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcBlockPublicAccessOptions:VpcBlockPublicAccessOptions"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "internetGatewayBlockMode".into(),
                    value: &internet_gateway_block_mode_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "awsRegion".into(),
                },
                register_interface::ResultField {
                    name: "internetGatewayBlockMode".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcBlockPublicAccessOptionsResult {
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            aws_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsRegion").unwrap(),
            ),
            internet_gateway_block_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetGatewayBlockMode").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
