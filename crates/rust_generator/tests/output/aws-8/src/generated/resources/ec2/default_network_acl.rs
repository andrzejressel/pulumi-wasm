/// Provides a resource to manage a VPC's default network ACL. This resource can manage the default network ACL of the default or a non-default VPC.
///
/// > **NOTE:** This is an advanced resource with special caveats. Please read this document in its entirety before using this resource. The `aws.ec2.DefaultNetworkAcl` behaves differently from normal resources. This provider does not _create_ this resource but instead attempts to "adopt" it into management.
///
/// Every VPC has a default network ACL that can be managed but not destroyed. When the provider first adopts the Default Network ACL, it **immediately removes all rules in the ACL**. It then proceeds to create any rules specified in the configuration. This step is required so that only the rules specified in the configuration are created.
///
/// This resource treats its inline rules as absolute; only the rules defined inline are created, and any additions/removals external to this resource will result in diffs being shown. For these reasons, this resource is incompatible with the `aws.ec2.NetworkAclRule` resource.
///
/// For more information about Network ACLs, see the AWS Documentation on [Network ACLs][aws-network-acls].
///
/// ## Example Usage
///
/// ### Basic Example
///
/// The following config gives the Default Network ACL the same rules that AWS includes but pulls the resource under management by this provider. This means that any ACL rules added or changed will be detected as drift.
///
/// ```yaml
/// resources:
///   mainvpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.1.0.0/16
///   default:
///     type: aws:ec2:DefaultNetworkAcl
///     properties:
///       defaultNetworkAclId: ${mainvpc.defaultNetworkAclId}
///       ingress:
///         - protocol: -1
///           ruleNo: 100
///           action: allow
///           cidrBlock: 0.0.0.0/0
///           fromPort: 0
///           toPort: 0
///       egress:
///         - protocol: -1
///           ruleNo: 100
///           action: allow
///           cidrBlock: 0.0.0.0/0
///           fromPort: 0
///           toPort: 0
/// ```
///
/// ### Example: Deny All Egress Traffic, Allow Ingress
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
///     type: aws:ec2:DefaultNetworkAcl
///     properties:
///       defaultNetworkAclId: ${mainvpc.defaultNetworkAclId}
///       ingress:
///         - protocol: -1
///           ruleNo: 100
///           action: allow
///           cidrBlock: ${mainvpcAwsDefaultVpc.cidrBlock}
///           fromPort: 0
///           toPort: 0
/// ```
///
/// ### Example: Deny All Traffic To Any Subnet In The Default Network ACL
///
/// This config denies all traffic in the Default ACL. This can be useful if you want to lock down the VPC to force all resources to assign a non-default ACL.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = default_network_acl::create(
///         "default",
///         DefaultNetworkAclArgs::builder()
///             .default_network_acl_id("${mainvpc.defaultNetworkAclId}")
///             .build_struct(),
///     );
///     let mainvpc = vpc::create(
///         "mainvpc",
///         VpcArgs::builder().cidr_block("10.1.0.0/16").build_struct(),
///     );
/// }
/// ```
///
/// ### Managing Subnets In A Default Network ACL
///
/// Within a VPC, all Subnets must be associated with a Network ACL. In order to "delete" the association between a Subnet and a non-default Network ACL, the association is destroyed by replacing it with an association between the Subnet and the Default ACL instead.
///
/// When managing the Default Network ACL, you cannot "remove" Subnets. Instead, they must be reassigned to another Network ACL, or the Subnet itself must be destroyed. Because of these requirements, removing the `subnet_ids` attribute from the configuration of a `aws.ec2.DefaultNetworkAcl` resource may result in a reoccurring plan, until the Subnets are reassigned to another Network ACL or are destroyed.
///
/// Because Subnets are by default associated with the Default Network ACL, any non-explicit association will show up as a plan to remove the Subnet. For example: if you have a custom `aws.ec2.NetworkAcl` with two subnets attached, and you remove the `aws.ec2.NetworkAcl` resource, after successfully destroying this resource future plans will show a diff on the managed `aws.ec2.DefaultNetworkAcl`, as those two Subnets have been orphaned by the now destroyed network acl and thus adopted by the Default Network ACL. In order to avoid a reoccurring plan, they will need to be reassigned, destroyed, or added to the `subnet_ids` attribute of the `aws.ec2.DefaultNetworkAcl` entry.
///
/// As an alternative to the above, you can also specify the following lifecycle configuration in your `aws.ec2.DefaultNetworkAcl` resource:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = default_network_acl::create(
///         "default",
///         DefaultNetworkAclArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Removing `aws.ec2.DefaultNetworkAcl` From Your Configuration
///
/// Each AWS VPC comes with a Default Network ACL that cannot be deleted. The `aws.ec2.DefaultNetworkAcl` allows you to manage this Network ACL, but the provider cannot destroy it. Removing this resource from your configuration will remove it from your statefile and management, **but will not destroy the Network ACL.** All Subnets associations and ingress or egress rules will be left as they are at the time of removal. You can resume managing them via the AWS Console.
///
/// ## Import
///
/// Using `pulumi import`, import Default Network ACLs using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/defaultNetworkAcl:DefaultNetworkAcl sample acl-7aaabd18
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_network_acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultNetworkAclArgs {
        /// Network ACL ID to manage. This attribute is exported from `aws.ec2.Vpc`, or manually found via the AWS Console.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub default_network_acl_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for an egress rule. Detailed below.
        #[builder(into, default)]
        pub egress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::DefaultNetworkAclEgress>>,
        >,
        /// Configuration block for an ingress rule. Detailed below.
        #[builder(into, default)]
        pub ingress: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::DefaultNetworkAclIngress>>,
        >,
        /// List of Subnet IDs to apply the ACL to. See the notes above on Managing Subnets in the Default Network ACL
        #[builder(into, default)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DefaultNetworkAclResult {
        /// ARN of the Default Network ACL
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Network ACL ID to manage. This attribute is exported from `aws.ec2.Vpc`, or manually found via the AWS Console.
        ///
        /// The following arguments are optional:
        pub default_network_acl_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for an egress rule. Detailed below.
        pub egress: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::DefaultNetworkAclEgress>>,
        >,
        /// Configuration block for an ingress rule. Detailed below.
        pub ingress: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ec2::DefaultNetworkAclIngress>>,
        >,
        /// ID of the AWS account that owns the Default Network ACL
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// List of Subnet IDs to apply the ACL to. See the notes above on Managing Subnets in the Default Network ACL
        pub subnet_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ID of the associated VPC
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultNetworkAclArgs,
    ) -> DefaultNetworkAclResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_network_acl_id_binding = args
            .default_network_acl_id
            .get_output(context);
        let egress_binding = args.egress.get_output(context);
        let ingress_binding = args.ingress.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/defaultNetworkAcl:DefaultNetworkAcl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultNetworkAclId".into(),
                    value: default_network_acl_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "egress".into(),
                    value: egress_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingress".into(),
                    value: ingress_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultNetworkAclResult {
            arn: o.get_field("arn"),
            default_network_acl_id: o.get_field("defaultNetworkAclId"),
            egress: o.get_field("egress"),
            ingress: o.get_field("ingress"),
            owner_id: o.get_field("ownerId"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
