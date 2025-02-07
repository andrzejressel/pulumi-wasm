/// Provides a AWS WorkLink Fleet resource.
///
/// !> **WARNING:** The `aws.worklink.Fleet` resource has been deprecated and will be removed in a future version. Use Amazon WorkSpaces Secure Browser instead.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
///
/// Network Configuration Usage:
///
///
/// Identity Provider Configuration Usage:
///
/// ```yaml
/// resources:
///   test:
///     type: aws:worklink:Fleet
///     properties:
///       name: tf-worklink-fleet
///       identityProvider:
///         type: SAML
///         samlMetadata:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: saml-metadata.xml
///             return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WorkLink using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:worklink/fleet:Fleet test arn:aws:worklink::123456789012:fleet/example
/// ```
pub mod fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetArgs {
        /// The ARN of the Amazon Kinesis data stream that receives the audit events. Kinesis data stream name must begin with `"AmazonWorkLink-"`.
        #[builder(into, default)]
        pub audit_stream_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.
        #[builder(into, default)]
        pub device_ca_certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the fleet.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provide this to allow manage the identity provider configuration for the fleet. Fields documented below.
        #[builder(into, default)]
        pub identity_provider: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::worklink::FleetIdentityProvider>,
        >,
        /// A region-unique name for the AMI.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Provide this to allow manage the company network configuration for the fleet. Fields documented below.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::worklink::FleetNetwork>,
        >,
        /// The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region. Defaults to `true`.
        ///
        /// **network** requires the following:
        ///
        /// > **NOTE:** `network` is cannot removed without force recreating.
        #[builder(into, default)]
        pub optimize_for_end_user_location: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetResult {
        /// The ARN of the created WorkLink Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the Amazon Kinesis data stream that receives the audit events. Kinesis data stream name must begin with `"AmazonWorkLink-"`.
        pub audit_stream_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier used by users to sign in to the Amazon WorkLink app.
        pub company_code: pulumi_gestalt_rust::Output<String>,
        /// The time that the fleet was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The certificate chain, including intermediate certificates and the root certificate authority certificate used to issue device certificates.
        pub device_ca_certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the fleet.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Provide this to allow manage the identity provider configuration for the fleet. Fields documented below.
        pub identity_provider: pulumi_gestalt_rust::Output<
            Option<super::super::types::worklink::FleetIdentityProvider>,
        >,
        /// The time that the fleet was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// A region-unique name for the AMI.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Provide this to allow manage the company network configuration for the fleet. Fields documented below.
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::worklink::FleetNetwork>,
        >,
        /// The option to optimize for better performance by routing traffic through the closest AWS Region to users, which may be outside of your home Region. Defaults to `true`.
        ///
        /// **network** requires the following:
        ///
        /// > **NOTE:** `network` is cannot removed without force recreating.
        pub optimize_for_end_user_location: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FleetArgs,
    ) -> FleetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let audit_stream_arn_binding = args
            .audit_stream_arn
            .get_output(context)
            .get_inner();
        let device_ca_certificate_binding = args
            .device_ca_certificate
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let identity_provider_binding = args
            .identity_provider
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let optimize_for_end_user_location_binding = args
            .optimize_for_end_user_location
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:worklink/fleet:Fleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auditStreamArn".into(),
                    value: &audit_stream_arn_binding,
                },
                register_interface::ObjectField {
                    name: "deviceCaCertificate".into(),
                    value: &device_ca_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "identityProvider".into(),
                    value: &identity_provider_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "optimizeForEndUserLocation".into(),
                    value: &optimize_for_end_user_location_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FleetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            audit_stream_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("auditStreamArn"),
            ),
            company_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("companyCode"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            device_ca_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceCaCertificate"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            identity_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityProvider"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            optimize_for_end_user_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("optimizeForEndUserLocation"),
            ),
        }
    }
}
