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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebsiteCertificateAuthorityAssociationArgs,
    ) -> WebsiteCertificateAuthorityAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_binding = args.certificate.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let fleet_arn_binding = args.fleet_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:worklink/websiteCertificateAuthorityAssociation:WebsiteCertificateAuthorityAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fleetArn".into(),
                    value: fleet_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebsiteCertificateAuthorityAssociationResult {
            certificate: o.get_field("certificate"),
            display_name: o.get_field("displayName"),
            fleet_arn: o.get_field("fleetArn"),
            website_ca_id: o.get_field("websiteCaId"),
        }
    }
}
