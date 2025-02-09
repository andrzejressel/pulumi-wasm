/// Provides a AWS WorkLink Website Certificate Authority Association resource.
///
/// !> **WARNING:** The `aws.worklink.WebsiteCertificateAuthorityAssociation` resource has been deprecated and will be removed in a future version. Use Amazon WorkSpaces Secure Browser instead.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:worklink:Fleet
///     properties:
///       name: example
///   test:
///     type: aws:worklink:WebsiteCertificateAuthorityAssociation
///     properties:
///       fleetArn: ${testAwsWorklinkFleet.arn}
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: certificate.pem
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WorkLink Website Certificate Authority using `FLEET-ARN,WEBSITE-CA-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:worklink/websiteCertificateAuthorityAssociation:WebsiteCertificateAuthorityAssociation example arn:aws:worklink::123456789012:fleet/example,abcdefghijk
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod website_certificate_authority_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebsiteCertificateAuthorityAssociationArgs {
        /// The root certificate of the Certificate Authority.
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The certificate name to display.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the fleet.
        #[builder(into)]
        pub fleet_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WebsiteCertificateAuthorityAssociationResult {
        /// The root certificate of the Certificate Authority.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The certificate name to display.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the fleet.
        pub fleet_arn: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the Certificate Authority.
        pub website_ca_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WebsiteCertificateAuthorityAssociationArgs,
    ) -> WebsiteCertificateAuthorityAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_binding_1 = args.certificate.get_output(context);
        let certificate_binding = certificate_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let fleet_arn_binding_1 = args.fleet_arn.get_output(context);
        let fleet_arn_binding = fleet_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:worklink/websiteCertificateAuthorityAssociation:WebsiteCertificateAuthorityAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "fleetArn".into(),
                    value: &fleet_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WebsiteCertificateAuthorityAssociationResult {
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            fleet_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fleetArn"),
            ),
            website_ca_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("websiteCaId"),
            ),
        }
    }
}
