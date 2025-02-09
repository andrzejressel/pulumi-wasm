/// Provides a resource to manage a default security group. This resource can manage the default security group of the default or a non-default VPC.
///
/// > **NOTE:** This is an advanced resource with special caveats. Please read this document in its entirety before using this resource. The `aws.ec2.DefaultSecurityGroup` resource behaves differently from normal resources. This provider does not _create_ this resource but instead attempts to "adopt" it into management.
///
/// When the provider first begins managing the default security group, it **immediately removes all ingress and egress rules in the Security Group**. It then creates any rules specified in the configuration. This way only the rules specified in the configuration are created.
///
/// This resource treats its inline rules as absolute; only the rules defined inline are created, and any additions/removals external to this resource will result in diff shown. For these reasons, this resource is incompatible with the `aws.ec2.SecurityGroupRule` resource.
///
/// For more information about default security groups, see the AWS documentation on [Default Security Groups][aws-default-security-groups]. To manage normal security groups, see the `aws.ec2.SecurityGroup` resource.
///
/// ## Example Usage
///
/// The following config gives the default security group the same rules that AWS provides by default but under management by this provider. This means that any ingress or egress rules added or changed will be detected as drift.
///
/// ```yaml
/// resources:
///   mainvpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.1.0.0/16
///   default:
///     type: aws:ec2:DefaultSecurityGroup
///     properties:
///       vpcId: ${mainvpc.id}
///       ingress:
///         - protocol: -1
///           self: true
///           fromPort: 0
///           toPort: 0
///       egress:
///         - fromPort: 0
///           toPort: 0
///           protocol: '-1'
///           cidrBlocks:
///             - 0.0.0.0/0
/// ```
///
/// ### Example Config To Deny All Egress Traffic, Allowing Ingress
///
/// The following denies all Egress traffic by omitting any `egress` rules, while including the default `ingress` rule to allow all traffic.
///
/// ```yaml
/// resources:
///   mainvpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.1.0.0/16
///   default:
///     type: aws:ec2:DefaultSecurityGroup
///     properties:
///       vpcId: ${mainvpc.id}
///       ingress:
///         - protocol: -1
///           self: true
///           fromPort: 0
///           toPort: 0
/// ```
///
/// ### Removing `aws.ec2.DefaultSecurityGroup` From Your Configuration
///
/// Removing this resource from your configuration will remove it from your statefile and management, but will not destroy the Security Group. All ingress or egress rules will be left as they are at the time of removal. You can resume managing them via the AWS Console.
///
/// ## Import
///
/// Using `pulumi import`, import Security Groups using the security group `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultSecurityGroup:DefaultSecurityGroup default_sg sg-903004f8
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_security_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultSecurityGroupArgs {
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub egress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::DefaultSecurityGroupEgress>>,
        >,
        /// Configuration block. Detailed below.
        #[builder(into, default)]
        pub ingress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::DefaultSecurityGroupIngress>>,
        >,
        #[builder(into, default)]
        pub revoke_rules_on_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC ID. **Note that changing the `vpc_id` will _not_ restore any default security group rules that were modified, added, or removed.** It will be left in its current state.
        #[builder(into, default)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DefaultSecurityGroupResult {
        /// ARN of the security group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the security group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Configuration block. Detailed below.
        pub egress: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::DefaultSecurityGroupEgress>,
        >,
        /// Configuration block. Detailed below.
        pub ingress: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::DefaultSecurityGroupIngress>,
        >,
        /// Name of the security group.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// Owner ID.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        pub revoke_rules_on_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC ID. **Note that changing the `vpc_id` will _not_ restore any default security group rules that were modified, added, or removed.** It will be left in its current state.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DefaultSecurityGroupArgs,
    ) -> DefaultSecurityGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let egress_binding_1 = args.egress.get_output(context);
        let egress_binding = egress_binding_1.get_inner();
        let ingress_binding_1 = args.ingress.get_output(context);
        let ingress_binding = ingress_binding_1.get_inner();
        let revoke_rules_on_delete_binding_1 = args
            .revoke_rules_on_delete
            .get_output(context);
        let revoke_rules_on_delete_binding = revoke_rules_on_delete_binding_1
            .get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/defaultSecurityGroup:DefaultSecurityGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "egress".into(),
                    value: &egress_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "revokeRulesOnDelete".into(),
                    value: &revoke_rules_on_delete_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefaultSecurityGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            egress: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("egress"),
            ),
            ingress: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ingress"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            revoke_rules_on_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revokeRulesOnDelete"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
