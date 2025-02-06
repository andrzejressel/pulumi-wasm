/// Authorizes the Shield Response Team (SRT) using the specified role, to access your AWS account to assist with DDoS attack mitigation during potential attacks.
/// For more information see [Configure AWS SRT Support](https://docs.aws.amazon.com/waf/latest/developerguide/authorize-srt.html)
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:shield:DrtAccessRoleArnAssociation
///     properties:
///       roleArn: ${exampleRole.arn}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: ""
///               Effect: Allow
///               Principal:
///                 Service: drt.shield.amazonaws.com
///               Action: sts:AssumeRole
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield DRT access role ARN association using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:shield/drtAccessRoleArnAssociation:DrtAccessRoleArnAssociation example 123456789012
/// ```
pub mod drt_access_role_arn_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DrtAccessRoleArnAssociationArgs {
        /// The Amazon Resource Name (ARN) of the role the SRT will use to access your AWS account. Prior to making the AssociateDRTRole request, you must attach the `AWSShieldDRTAccessPolicy` managed policy to this role.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::shield::DrtAccessRoleArnAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DrtAccessRoleArnAssociationResult {
        /// The Amazon Resource Name (ARN) of the role the SRT will use to access your AWS account. Prior to making the AssociateDRTRole request, you must attach the `AWSShieldDRTAccessPolicy` managed policy to this role.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::shield::DrtAccessRoleArnAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DrtAccessRoleArnAssociationArgs,
    ) -> DrtAccessRoleArnAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/drtAccessRoleArnAssociation:DrtAccessRoleArnAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DrtAccessRoleArnAssociationResult {
            role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
