/// Manages an IoT fleet provisioning template. For more info, see the AWS documentation on [fleet provisioning](https://docs.aws.amazon.com/iot/latest/developerguide/provision-wo-cert.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   iotFleetProvisioning:
///     type: aws:iam:Role
///     name: iot_fleet_provisioning
///     properties:
///       name: IoTProvisioningServiceRole
///       path: /service-role/
///       assumeRolePolicy: ${iotAssumeRolePolicy.json}
///   iotFleetProvisioningRegistration:
///     type: aws:iam:RolePolicyAttachment
///     name: iot_fleet_provisioning_registration
///     properties:
///       role: ${iotFleetProvisioning.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSIoTThingsRegistration
///   devicePolicyPolicy:
///     type: aws:iot:Policy
///     name: device_policy
///     properties:
///       name: DevicePolicy
///       policy: ${devicePolicy.json}
///   fleet:
///     type: aws:iot:ProvisioningTemplate
///     properties:
///       name: FleetTemplate
///       description: My provisioning template
///       provisioningRoleArn: ${iotFleetProvisioning.arn}
///       enabled: true
///       templateBody:
///         fn::toJSON:
///           Parameters:
///             SerialNumber:
///               Type: String
///           Resources:
///             certificate:
///               Properties:
///                 CertificateId:
///                   Ref: AWS::IoT::Certificate::Id
///                 Status: Active
///               Type: AWS::IoT::Certificate
///             policy:
///               Properties:
///                 PolicyName: ${devicePolicyPolicy.name}
///               Type: AWS::IoT::Policy
/// variables:
///   iotAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - iot.amazonaws.com
///   devicePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - iot:Subscribe
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IoT fleet provisioning templates using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iot/provisioningTemplate:ProvisioningTemplate fleet FleetProvisioningTemplate
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod provisioning_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisioningTemplateArgs {
        /// The description of the fleet provisioning template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// True to enable the fleet provisioning template, otherwise false.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the fleet provisioning template.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a pre-provisioning hook template. Details below.
        #[builder(into, default)]
        pub pre_provisioning_hook: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iot::ProvisioningTemplatePreProvisioningHook>,
        >,
        /// The role ARN for the role associated with the fleet provisioning template. This IoT role grants permission to provision a device.
        #[builder(into)]
        pub provisioning_role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The JSON formatted contents of the fleet provisioning template.
        #[builder(into)]
        pub template_body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type you define in a provisioning template.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProvisioningTemplateResult {
        /// The ARN that identifies the provisioning template.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The default version of the fleet provisioning template.
        pub default_version_id: pulumi_gestalt_rust::Output<i32>,
        /// The description of the fleet provisioning template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// True to enable the fleet provisioning template, otherwise false.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the fleet provisioning template.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a pre-provisioning hook template. Details below.
        pub pre_provisioning_hook: pulumi_gestalt_rust::Output<
            Option<super::super::types::iot::ProvisioningTemplatePreProvisioningHook>,
        >,
        /// The role ARN for the role associated with the fleet provisioning template. This IoT role grants permission to provision a device.
        pub provisioning_role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The JSON formatted contents of the fleet provisioning template.
        pub template_body: pulumi_gestalt_rust::Output<String>,
        /// The type you define in a provisioning template.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProvisioningTemplateArgs,
    ) -> ProvisioningTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let pre_provisioning_hook_binding = args
            .pre_provisioning_hook
            .get_output(context)
            .get_inner();
        let provisioning_role_arn_binding = args
            .provisioning_role_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let template_body_binding = args.template_body.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/provisioningTemplate:ProvisioningTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "preProvisioningHook".into(),
                    value: &pre_provisioning_hook_binding,
                },
                register_interface::ObjectField {
                    name: "provisioningRoleArn".into(),
                    value: &provisioning_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateBody".into(),
                    value: &template_body_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProvisioningTemplateResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            default_version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultVersionId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pre_provisioning_hook: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preProvisioningHook"),
            ),
            provisioning_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisioningRoleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            template_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateBody"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
