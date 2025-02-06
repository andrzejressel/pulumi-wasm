/// Resource for enabling private DNS on an AWS VPC (Virtual Private Cloud) Endpoint.
///
/// > When using this resource, the `private_dns_enabled` argument should be omitted on the parent `aws.ec2.VpcEndpoint` resource.
/// Setting the value both places can lead to unintended behavior and persistent differences.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = endpoint_private_dns::create(
///         "example",
///         EndpointPrivateDnsArgs::builder()
///             .private_dns_enabled(true)
///             .vpc_endpoint_id("${exampleAwsVpcEndpoint.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a VPC (Virtual Private Cloud) Endpoint Private DNS using the `vpc_endpoint_id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpc/endpointPrivateDns:EndpointPrivateDns example vpce-abcd-1234
/// ```
pub mod endpoint_private_dns {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointPrivateDnsArgs {
        /// Indicates whether a private hosted zone is associated with the VPC. Only applicable for `Interface` endpoints.
        #[builder(into)]
        pub private_dns_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// VPC endpoint identifier.
        #[builder(into)]
        pub vpc_endpoint_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointPrivateDnsResult {
        /// Indicates whether a private hosted zone is associated with the VPC. Only applicable for `Interface` endpoints.
        pub private_dns_enabled: pulumi_gestalt_rust::Output<bool>,
        /// VPC endpoint identifier.
        pub vpc_endpoint_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointPrivateDnsArgs,
    ) -> EndpointPrivateDnsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let private_dns_enabled_binding = args
            .private_dns_enabled
            .get_output(context)
            .get_inner();
        let vpc_endpoint_id_binding = args
            .vpc_endpoint_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/endpointPrivateDns:EndpointPrivateDns".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "privateDnsEnabled".into(),
                    value: &private_dns_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointPrivateDnsResult {
            private_dns_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateDnsEnabled"),
            ),
            vpc_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcEndpointId"),
            ),
        }
    }
}
