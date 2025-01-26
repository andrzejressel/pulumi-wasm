/// Provides a security group resource.
///
/// > **NOTE:** Avoid using the `ingress` and `egress` arguments of the `aws.ec2.SecurityGroup` resource to configure in-line rules, as they struggle with managing multiple CIDR blocks, and, due to the historical lack of unique IDs, tags and descriptions. To avoid these problems, use the current best practice of the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources with one CIDR block per rule.
///
/// !> **WARNING:** You should not use the `aws.ec2.SecurityGroup` resource with _in-line rules_ (using the `ingress` and `egress` arguments of `aws.ec2.SecurityGroup`) in conjunction with the `aws.vpc.SecurityGroupEgressRule` and `aws.vpc.SecurityGroupIngressRule` resources or the `aws.ec2.SecurityGroupRule` resource. Doing so may cause rule conflicts, perpetual differences, and result in rules being overwritten.
///
/// > **NOTE:** Referencing Security Groups across VPC peering has certain restrictions. More information is available in the [VPC Peering User Guide](https://docs.aws.amazon.com/vpc/latest/peering/vpc-peering-security-groups.html).
///
/// > **NOTE:** Due to [AWS Lambda improved VPC networking changes that began deploying in September 2019](https://aws.amazon.com/blogs/compute/announcing-improved-vpc-networking-for-aws-lambda-functions/), security groups associated with Lambda Functions can take up to 45 minutes to successfully delete. To allow for successful deletion, the provider will wait for at least 45 minutes even if a shorter delete timeout is specified.
///
/// > **NOTE:** The `cidr_blocks` and `ipv6_cidr_blocks` parameters are optional in the `ingress` and `egress` blocks. If nothing is specified, traffic will be blocked as described in _NOTE on Egress rules_ later.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   allowTls:
///     type: aws:ec2:SecurityGroup
///     name: allow_tls
///     properties:
///       name: allow_tls
///       description: Allow TLS inbound traffic and all outbound traffic
///       vpcId: ${main.id}
///       tags:
///         Name: allow_tls
///   allowTlsIpv4:
///     type: aws:vpc:SecurityGroupIngressRule
///     name: allow_tls_ipv4
///     properties:
///       securityGroupId: ${allowTls.id}
///       cidrIpv4: ${main.cidrBlock}
///       fromPort: 443
///       ipProtocol: tcp
///       toPort: 443
///   allowTlsIpv6:
///     type: aws:vpc:SecurityGroupIngressRule
///     name: allow_tls_ipv6
///     properties:
///       securityGroupId: ${allowTls.id}
///       cidrIpv6: ${main.ipv6CidrBlock}
///       fromPort: 443
///       ipProtocol: tcp
///       toPort: 443
///   allowAllTrafficIpv4:
///     type: aws:vpc:SecurityGroupEgressRule
///     name: allow_all_traffic_ipv4
///     properties:
///       securityGroupId: ${allowTls.id}
///       cidrIpv4: 0.0.0.0/0
///       ipProtocol: '-1'
///   allowAllTrafficIpv6:
///     type: aws:vpc:SecurityGroupEgressRule
///     name: allow_all_traffic_ipv6
///     properties:
///       securityGroupId: ${allowTls.id}
///       cidrIpv6: ::/0
///       ipProtocol: '-1'
/// ```
///
/// > **NOTE on Egress rules:** By default, AWS creates an `ALLOW ALL` egress rule when creating a new Security Group inside of a VPC. When creating a new Security Group inside a VPC, **this provider will remove this default rule**, and require you specifically re-create it if you desire that rule. We feel this leads to fewer surprises in terms of controlling your egress rules. If you desire this rule to be in place, you can use this `egress` block:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group::create(
///         "example",
///         SecurityGroupArgs::builder()
///             .egress(
///                 vec![
///                     SecurityGroupEgress::builder().cidrBlocks(vec!["0.0.0.0/0",])
///                     .fromPort(0).ipv6CidrBlocks(vec!["::/0",]).protocol("-1").toPort(0)
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage With Prefix List IDs
///
/// Prefix Lists are either managed by AWS internally, or created by the customer using a
/// Prefix List resource. Prefix Lists provided by
/// AWS are associated with a prefix list name, or service name, that is linked to a specific region.
/// Prefix list IDs are exported on VPC Endpoints, so you can use this format:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group::create(
///         "example",
///         SecurityGroupArgs::builder()
///             .egress(
///                 vec![
///                     SecurityGroupEgress::builder().fromPort(0)
///                     .prefixListIds(vec!["${myEndpoint.prefixListId}",]).protocol("-1")
///                     .toPort(0).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let myEndpoint = vpc_endpoint::create(
///         "myEndpoint",
///         VpcEndpointArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// You can also find a specific Prefix List using the `aws.ec2.getPrefixList` data source.
///
/// ### Removing All Ingress and Egress Rules
///
/// The `ingress` and `egress` arguments are processed in attributes-as-blocks mode. Due to this, removing these arguments from the configuration will **not** cause the provider to destroy the managed rules. To subsequently remove all managed ingress and egress rules:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group::create(
///         "example",
///         SecurityGroupArgs::builder()
///             .egress(vec![])
///             .ingress(vec![])
///             .name("sg")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Recreating a Security Group
///
/// A simple security group `name` change "forces new" the security group--the provider destroys the security group and creates a new one. (Likewise, `description`, `name_prefix`, or `vpc_id` [cannot be changed](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/working-with-security-groups.html#creating-security-group).) Attempting to recreate the security group leads to a variety of complications depending on how it is used.
///
/// Security groups are generally associated with other resources--**more than 100** AWS Provider resources reference security groups. Referencing a resource from another resource creates a one-way dependency. For example, if you create an EC2 `aws.ec2.Instance` that has a `vpc_security_group_ids` argument that refers to an `aws.ec2.SecurityGroup` resource, the `aws.ec2.SecurityGroup` is a dependent of the `aws.ec2.Instance`. Because of this, the provider will create the security group first so that it can then be associated with the EC2 instance.
///
/// However, the dependency relationship actually goes both directions causing the _Security Group Deletion Problem_. AWS does not allow you to delete the security group associated with another resource (_e.g._, the `aws.ec2.Instance`).
///
/// The provider does not model bi-directional dependencies like this, but, even if it did, simply knowing the dependency situation would not be enough to solve it. For example, some resources must always have an associated security group while others don't need to. In addition, when the `aws.ec2.SecurityGroup` resource attempts to recreate, it receives a dependent object error, which does not provide information on whether the dependent object is a security group rule or, for example, an associated EC2 instance. Within the provider, the associated resource (_e.g._, `aws.ec2.Instance`) does not receive an error when the `aws.ec2.SecurityGroup` is trying to recreate even though that is where changes to the associated resource would need to take place (_e.g._, removing the security group association).
///
/// Despite these sticky problems, below are some ways to improve your experience when you find it necessary to recreate a security group.
///
/// ### Shorter timeout
///
/// (This example is one approach to recreating security groups. For more information on the challenges and the _Security Group Deletion Problem_, see the section above.)
///
/// If destroying a security group takes a long time, it may be because the provider cannot distinguish between a dependent object (_e.g._, a security group rule or EC2 instance) that is _in the process of being deleted_ and one that is not. In other words, it may be waiting for a train that isn't scheduled to arrive. To fail faster, shorten the `delete` timeout from the default timeout:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group::create(
///         "example",
///         SecurityGroupArgs::builder().name("izizavle").build_struct(),
///     );
/// }
/// ```
///
/// ### Provisioners
///
/// (This example is one approach to recreating security groups. For more information on the challenges and the _Security Group Deletion Problem_, see the section above.)
///
/// **DISCLAIMER:** We **_HIGHLY_** recommend using one of the above approaches and _NOT_ using local provisioners. Provisioners, like the one shown below, should be considered a **last resort** since they are _not readable_, _require skills outside standard configuration_, are _error prone_ and _difficult to maintain_, are not compatible with cloud environments and upgrade tools, require AWS CLI installation, and are subject to changes outside the AWS Provider.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:SecurityGroup
///     properties:
///       name: sg
///       tags:
///         workaround1: tagged-name
///         workaround2: ${default.id}
///   exampleProvisioner0:
///     type: command:local:Command
///     properties:
///       create: 'true'
///       update: 'true'
///       delete: |2
///                     ENDPOINT_ID=`aws ec2 describe-vpc-endpoints --filters "Name=tag:Name,Values=${tags.workaround1}" --query "VpcEndpoints[0].VpcEndpointId" --output text` &&
///                     aws ec2 modify-vpc-endpoint --vpc-endpoint-id $${ENDPOINT_ID} --add-security-group-ids ${tags.workaround2} --remove-security-group-ids ${id}
///     options:
///       dependsOn:
///         - ${example}
///   exampleResource:
///     type: null:Resource
///     name: example
///     properties:
///       triggers:
///         rerun_upon_change_of:
///           fn::invoke:
///             function: std:join
///             arguments:
///               separator: ','
///               input: ${exampleAwsVpcEndpoint.securityGroupIds}
///             return: result
///   exampleResourceProvisioner0:
///     type: command:local:Command
///     properties:
///       create: |2
///                     aws ec2 modify-vpc-endpoint --vpc-endpoint-id ${exampleAwsVpcEndpoint.id} --remove-security-group-ids ${default.id}
///     options:
///       dependsOn:
///         - ${exampleResource}
/// variables:
///   default:
///     fn::invoke:
///       function: aws:ec2:getSecurityGroup
///       arguments:
///         name: default
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Groups using the security group `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/securityGroup:SecurityGroup elb_sg sg-903004f8
/// ```
pub mod security_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupArgs {
        /// Security group description. Defaults to `Managed by Pulumi`. Cannot be `""`. **NOTE**: This field maps to the AWS `GroupDescription` attribute, for which there is no Update API. If you'd like to classify your security groups in a way that can be updated, use `tags`.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block for egress rules. Can be specified multiple times for each egress rule. Each egress block supports fields documented below. This argument is processed in attribute-as-blocks mode.
        #[builder(into, default)]
        pub egress: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SecurityGroupEgress>>,
        >,
        /// Configuration block for ingress rules. Can be specified multiple times for each ingress rule. Each ingress block supports fields documented below. This argument is processed in attribute-as-blocks mode.
        #[builder(into, default)]
        pub ingress: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::SecurityGroupIngress>>,
        >,
        /// Name of the security group. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Instruct the provider to revoke all of the Security Groups attached ingress and egress rules before deleting the rule itself. This is normally not needed, however certain AWS services such as Elastic Map Reduce may automatically add required rules to security groups used with the service, and those rules may contain a cyclic dependency that prevent the security groups from being destroyed without removing the dependency first. Default `false`.
        #[builder(into, default)]
        pub revoke_rules_on_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// VPC ID. Defaults to the region's default VPC.
        #[builder(into, default)]
        pub vpc_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupResult {
        /// ARN of the security group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Security group description. Defaults to `Managed by Pulumi`. Cannot be `""`. **NOTE**: This field maps to the AWS `GroupDescription` attribute, for which there is no Update API. If you'd like to classify your security groups in a way that can be updated, use `tags`.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Configuration block for egress rules. Can be specified multiple times for each egress rule. Each egress block supports fields documented below. This argument is processed in attribute-as-blocks mode.
        pub egress: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::SecurityGroupEgress>,
        >,
        /// Configuration block for ingress rules. Can be specified multiple times for each ingress rule. Each ingress block supports fields documented below. This argument is processed in attribute-as-blocks mode.
        pub ingress: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::SecurityGroupIngress>,
        >,
        /// Name of the security group. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// Owner ID.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Instruct the provider to revoke all of the Security Groups attached ingress and egress rules before deleting the rule itself. This is normally not needed, however certain AWS services such as Elastic Map Reduce may automatically add required rules to security groups used with the service, and those rules may contain a cyclic dependency that prevent the security groups from being destroyed without removing the dependency first. Default `false`.
        pub revoke_rules_on_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// VPC ID. Defaults to the region's default VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SecurityGroupArgs,
    ) -> SecurityGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let egress_binding = args.egress.get_output(context).get_inner();
        let ingress_binding = args.ingress.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let revoke_rules_on_delete_binding = args
            .revoke_rules_on_delete
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_id_binding = args.vpc_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/securityGroup:SecurityGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "egress".into(),
                    value: &egress_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "egress".into(),
                },
                register_interface::ResultField {
                    name: "ingress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "revokeRulesOnDelete".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecurityGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            egress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egress").unwrap(),
            ),
            ingress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            revoke_rules_on_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revokeRulesOnDelete").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
        }
    }
}
