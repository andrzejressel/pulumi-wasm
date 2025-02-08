/// Provides a License Manager association.
///
/// > **Note:** License configurations can also be associated with launch templates by specifying the `license_specifications` block for an `aws.ec2.LaunchTemplate`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleInstance:
///     type: aws:ec2:Instance
///     name: example
///     properties:
///       ami: ${example.id}
///       instanceType: t2.micro
///   exampleLicenseConfiguration:
///     type: aws:licensemanager:LicenseConfiguration
///     name: example
///     properties:
///       name: Example
///       licenseCountingType: Instance
///   exampleAssociation:
///     type: aws:licensemanager:Association
///     name: example
///     properties:
///       licenseConfigurationArn: ${exampleLicenseConfiguration.arn}
///       resourceArn: ${exampleInstance.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getAmi
///       arguments:
///         mostRecent: true
///         owners:
///           - amazon
///         filters:
///           - name: name
///             values:
///               - amzn-ami-vpc-nat*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import license configurations using `resource_arn,license_configuration_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:licensemanager/association:Association example arn:aws:ec2:eu-west-1:123456789012:image/ami-123456789abcdef01,arn:aws:license-manager:eu-west-1:123456789012:license-configuration:lic-0123456789abcdef0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssociationArgs {
        /// ARN of the license configuration.
        #[builder(into)]
        pub license_configuration_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the resource associated with the license configuration.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssociationResult {
        /// ARN of the license configuration.
        pub license_configuration_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the resource associated with the license configuration.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AssociationArgs,
    ) -> AssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let license_configuration_arn_binding = args
            .license_configuration_arn
            .get_output(context)
            .get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:licensemanager/association:Association".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "licenseConfigurationArn".into(),
                    value: &license_configuration_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AssociationResult {
            license_configuration_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseConfigurationArn"),
            ),
            resource_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}
