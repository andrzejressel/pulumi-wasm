/// Provides a resource to manage an [Amazon Detective Invitation Accepter](https://docs.aws.amazon.com/detective/latest/APIReference/API_AcceptInvitation.html). Ensure that the accepter is configured to use the AWS account you wish to _accept_ the invitation from the primary graph owner account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod invitation_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvitationAccepterArgs {
        /// ARN of the behavior graph that the member account is accepting the invitation for.
        #[builder(into)]
        pub graph_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InvitationAccepterResult {
        /// ARN of the behavior graph that the member account is accepting the invitation for.
        pub graph_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InvitationAccepterArgs,
    ) -> InvitationAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let graph_arn_binding = args.graph_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:detective/invitationAccepter:InvitationAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "graphArn".into(),
                    value: &graph_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InvitationAccepterResult {
            graph_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("graphArn"),
            ),
        }
    }
}
