/// Provides a Single Sign-On (SSO) ABAC Resource: https://docs.aws.amazon.com/singlesignon/latest/userguide/abac.html
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleInstanceAccessControlAttributes:
///     type: aws:ssoadmin:InstanceAccessControlAttributes
///     name: example
///     properties:
///       instanceArn: ${example.arns[0]}
///       attributes:
///         - key: name
///           values:
///             - sources:
///                 - $${path:name.givenName}
///         - key: last
///           values:
///             - sources:
///                 - $${path:name.familyName}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Account Assignments using the `instance_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributes example arn:aws:sso:::instance/ssoins-0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_access_control_attributes {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceAccessControlAttributesArgs {
        /// See AccessControlAttribute for more details.
        #[builder(into)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttribute>,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceAccessControlAttributesResult {
        /// See AccessControlAttribute for more details.
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttribute>,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub status_reason: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceAccessControlAttributesArgs,
    ) -> InstanceAccessControlAttributesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let instance_arn_binding = args.instance_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributes"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceAccessControlAttributesResult {
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            instance_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceArn"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
        }
    }
}
