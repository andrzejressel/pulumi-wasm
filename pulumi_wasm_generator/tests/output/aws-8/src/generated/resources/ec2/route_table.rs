/// Provides a resource to create a VPC routing table.
///
/// > **NOTE on Route Tables and Routes:** This provider currently
/// provides both a standalone Route resource and a Route Table resource with routes
/// defined in-line. At this time you cannot use a Route Table with in-line routes
/// in conjunction with any Route resources. Doing so will cause
/// a conflict of rule settings and will overwrite rules.
///
/// > **NOTE on `gateway_id` and `nat_gateway_id`:** The AWS API is very forgiving with these two
/// attributes and the `aws.ec2.RouteTable` resource can be created with a NAT ID specified as a Gateway ID attribute.
/// This _will_ lead to a permanent diff between your configuration and statefile, as the API returns the correct
/// parameters in the returned route table. If you're experiencing constant diffs in your `aws.ec2.RouteTable` resources,
/// the first thing to check is whether or not you're specifying a NAT ID instead of a Gateway ID, or vice-versa.
///
/// > **NOTE on `propagating_vgws` and the `aws.ec2.VpnGatewayRoutePropagation` resource:**
/// If the `propagating_vgws` argument is present, it's not supported to _also_
/// define route propagations using `aws.ec2.VpnGatewayRoutePropagation`, since
/// this resource will delete any propagating gateways not explicitly listed in
/// `propagating_vgws`. Omit this argument when defining route propagation using
/// the separate resource.
///
/// ## Example Usage
///
/// ### Basic example
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:RouteTable
///     properties:
///       vpcId: ${exampleAwsVpc.id}
///       routes:
///         - cidrBlock: 10.0.1.0/24
///           gatewayId: ${exampleAwsInternetGateway.id}
///         - ipv6CidrBlock: ::/0
///           egressOnlyGatewayId: ${exampleAwsEgressOnlyInternetGateway.id}
///       tags:
///         Name: example
/// ```
///
/// To subsequently remove all managed routes:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:RouteTable
///     properties:
///       vpcId: ${exampleAwsVpc.id}
///       routes: []
///       tags:
///         Name: example
/// ```
///
/// ### Adopting an existing local route
///
/// AWS creates certain routes that the AWS provider mostly ignores. You can manage them by importing or adopting them. See Import below for information on importing. This example shows adopting a route and then updating its target.
///
/// First, adopt an existing AWS-created route:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = vpc::create(
///         "test",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
///     let testRouteTable = route_table::create(
///         "testRouteTable",
///         RouteTableArgs::builder()
///             .routes(
///                 vec![
///                     RouteTableRoute::builder().cidrBlock("10.1.0.0/16")
///                     .gatewayId("local").build_struct(),
///                 ],
///             )
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Next, update the target of the route:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = vpc::create(
///         "test",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
///     let testNetworkInterface = network_interface::create(
///         "testNetworkInterface",
///         NetworkInterfaceArgs::builder().subnet_id("${testSubnet.id}").build_struct(),
///     );
///     let testRouteTable = route_table::create(
///         "testRouteTable",
///         RouteTableArgs::builder()
///             .routes(
///                 vec![
///                     RouteTableRoute::builder().cidrBlock("${test.cidrBlock}")
///                     .networkInterfaceId("${testNetworkInterface.id}").build_struct(),
///                 ],
///             )
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
///     let testSubnet = subnet::create(
///         "testSubnet",
///         SubnetArgs::builder()
///             .cidr_block("10.1.1.0/24")
///             .vpc_id("${test.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// The target could then be updated again back to `local`.
///
/// ## Import
///
/// Using `pulumi import`, import Route Tables using the route table `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/routeTable:RouteTable public_rt rtb-4e616f6d69
/// ```
pub mod route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableArgs {
        /// A list of virtual gateways for propagation.
        #[builder(into, default)]
        pub propagating_vgws: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of route objects. Their keys are documented below.
        /// This means that omitting this argument is interpreted as ignoring any existing routes. To remove all managed routes an empty list should be specified. See the example above.
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::RouteTableRoute>>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The VPC ID.
        #[builder(into)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteTableResult {
        /// The ARN of the route table.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the route table.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// A list of virtual gateways for propagation.
        pub propagating_vgws: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of route objects. Their keys are documented below.
        /// This means that omitting this argument is interpreted as ignoring any existing routes. To remove all managed routes an empty list should be specified. See the example above.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::RouteTableRoute>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The VPC ID.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouteTableArgs,
    ) -> RouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let propagating_vgws_binding = args
            .propagating_vgws
            .get_output(context)
            .get_inner();
        let routes_binding = args.routes.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/routeTable:RouteTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "propagatingVgws".into(),
                    value: &propagating_vgws_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "propagatingVgws".into(),
                },
                register_interface::ResultField {
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteTableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            propagating_vgws: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("propagatingVgws").unwrap(),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
