/// Provides a resource to manage an [Amazon Detective Invitation Accepter](https://docs.aws.amazon.com/detective/latest/APIReference/API_AcceptInvitation.html). Ensure that the accepter is configured to use the AWS account you wish to _accept_ the invitation from the primary graph owner account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = invitation_accepter::create(
///         "member",
///         InvitationAccepterArgs::builder().graph_arn("${primary.graphArn}").build_struct(),
///     );
///     let primary = graph::create("primary", GraphArgs::builder().build_struct());
///     let primaryMember = member::create(
///         "primaryMember",
///         MemberArgs::builder()
///             .account_id("ACCOUNT ID")
///             .email_address("EMAIL")
///             .graph_arn("${primary.id}")
///             .message("Message of the invite")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_detective_invitation_accepter` using the graph ARN. For example:
///
/// ```sh
/// $ pulumi import aws:detective/invitationAccepter:InvitationAccepter example arn:aws:detective:us-east-1:123456789101:graph:231684d34gh74g4bae1dbc7bd807d02d
/// ```
pub mod invitation_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvitationAccepterArgs {
        /// ARN of the behavior graph that the member account is accepting the invitation for.
        #[builder(into)]
        pub graph_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InvitationAccepterResult {
        /// ARN of the behavior graph that the member account is accepting the invitation for.
        pub graph_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InvitationAccepterArgs) -> InvitationAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let graph_arn_binding = args.graph_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:detective/invitationAccepter:InvitationAccepter".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "graphArn".into(),
                    value: &graph_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "graphArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InvitationAccepterResult {
            graph_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("graphArn").unwrap(),
            ),
        }
    }
}