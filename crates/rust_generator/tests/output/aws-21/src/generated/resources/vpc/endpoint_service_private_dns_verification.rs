/// Resource for managing an AWS VPC (Virtual Private Cloud) Endpoint Service Private DNS Verification.
/// This resource begins the verification process by calling the [`StartVpcEndpointServicePrivateDnsVerification`](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_StartVpcEndpointServicePrivateDnsVerification.html) API.
/// The service provider should add a record to the DNS server _before_ creating this resource.
///
/// For additional details, refer to the AWS documentation on [managing VPC endpoint service DNS names](https://docs.aws.amazon.com/vpc/latest/privatelink/manage-dns-names.html).
///
/// > Destruction of this resource will not stop the verification process, only remove the resource from state.
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
///     let example = endpoint_service_private_dns_verification::create(
///         "example",
///         EndpointServicePrivateDnsVerificationArgs::builder()
///             .service_id("${exampleAwsVpcEndpointService.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod endpoint_service_private_dns_verification {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointServicePrivateDnsVerificationArgs {
        /// ID of the endpoint service.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::vpc::EndpointServicePrivateDnsVerificationTimeouts,
            >,
        >,
        /// Whether to wait until the endpoint service returns a `Verified` status for the configured private DNS name.
        #[builder(into, default)]
        pub wait_for_verification: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EndpointServicePrivateDnsVerificationResult {
        /// ID of the endpoint service.
        ///
        /// The following arguments are optional:
        pub service_id: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::vpc::EndpointServicePrivateDnsVerificationTimeouts,
            >,
        >,
        /// Whether to wait until the endpoint service returns a `Verified` status for the configured private DNS name.
        pub wait_for_verification: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EndpointServicePrivateDnsVerificationArgs,
    ) -> EndpointServicePrivateDnsVerificationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let service_id_binding_1 = args.service_id.get_output(context);
        let service_id_binding = service_id_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let wait_for_verification_binding_1 = args
            .wait_for_verification
            .get_output(context);
        let wait_for_verification_binding = wait_for_verification_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/endpointServicePrivateDnsVerification:EndpointServicePrivateDnsVerification"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "waitForVerification".into(),
                    value: &wait_for_verification_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointServicePrivateDnsVerificationResult {
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            wait_for_verification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("waitForVerification"),
            ),
        }
    }
}
