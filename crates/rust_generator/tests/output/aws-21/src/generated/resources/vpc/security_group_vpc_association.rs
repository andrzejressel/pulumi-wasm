/// Resource for managing Security Group VPC Associations.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = security_group_vpc_association::create(
///         "example",
///         SecurityGroupVpcAssociationArgs::builder()
///             .security_group_id("sg-05f1f54ab49bb39a3")
///             .vpc_id("vpc-01df9d105095412ba")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a Security Group VPC Association using the `security_group_id` and `vpc_id` arguments, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:vpc/securityGroupVpcAssociation:SecurityGroupVpcAssociation example sg-12345,vpc-67890
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_group_vpc_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityGroupVpcAssociationArgs {
        /// The ID of the security group.
        #[builder(into)]
        pub security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vpc::SecurityGroupVpcAssociationTimeouts>,
        >,
        /// The ID of the VPC to make the association with.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SecurityGroupVpcAssociationResult {
        /// The ID of the security group.
        pub security_group_id: pulumi_gestalt_rust::Output<String>,
        /// State of the VPC association. See the [AWS documentation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SecurityGroupVpcAssociation.html) for possible values.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::vpc::SecurityGroupVpcAssociationTimeouts>,
        >,
        /// The ID of the VPC to make the association with.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SecurityGroupVpcAssociationArgs,
    ) -> SecurityGroupVpcAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let security_group_id_binding_1 = args.security_group_id.get_output(context);
        let security_group_id_binding = security_group_id_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let vpc_id_binding_1 = args.vpc_id.get_output(context);
        let vpc_id_binding = vpc_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpc/securityGroupVpcAssociation:SecurityGroupVpcAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "securityGroupId".into(),
                    value: &security_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "vpcId".into(),
                    value: &vpc_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SecurityGroupVpcAssociationResult {
            security_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroupId"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
