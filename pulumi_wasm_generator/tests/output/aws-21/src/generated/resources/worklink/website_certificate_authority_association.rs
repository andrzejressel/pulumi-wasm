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
pub mod website_certificate_authority_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebsiteCertificateAuthorityAssociationArgs {
        /// The root certificate of the Certificate Authority.
        #[builder(into)]
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The certificate name to display.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the fleet.
        #[builder(into)]
        pub fleet_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WebsiteCertificateAuthorityAssociationResult {
        /// The root certificate of the Certificate Authority.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The certificate name to display.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the fleet.
        pub fleet_arn: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the Certificate Authority.
        pub website_ca_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WebsiteCertificateAuthorityAssociationArgs,
    ) -> WebsiteCertificateAuthorityAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_binding = args.certificate.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let fleet_arn_binding = args.fleet_arn.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "fleetArn".into(),
                },
                register_interface::ResultField {
                    name: "websiteCaId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WebsiteCertificateAuthorityAssociationResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            fleet_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetArn").unwrap(),
            ),
            website_ca_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("websiteCaId").unwrap(),
            ),
        }
    }
}
