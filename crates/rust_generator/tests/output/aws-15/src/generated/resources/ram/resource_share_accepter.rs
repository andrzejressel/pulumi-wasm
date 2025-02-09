/// Manage accepting a Resource Access Manager (RAM) Resource Share invitation. From a _receiver_ AWS account, accept an invitation to share resources that were shared by a _sender_ AWS account. To create a resource share in the _sender_, see the `aws.ram.ResourceShare` resource.
///
/// > **Note:** If both AWS accounts are in the same Organization and [RAM Sharing with AWS Organizations is enabled](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs), this resource is not necessary as RAM Resource Share invitations are not used.
///
/// ## Example Usage
///
/// This configuration provides an example of using multiple AWS providers to configure two different AWS accounts. In the _sender_ account, the configuration creates a `aws.ram.ResourceShare` and uses a data source in the _receiver_ account to create a `aws.ram.PrincipalAssociation` resource with the _receiver's_ account ID. In the _receiver_ account, the configuration accepts the invitation to share resources with the `aws.ram.ResourceShareAccepter`.
///
/// ```yaml
/// resources:
///   senderShare:
///     type: aws:ram:ResourceShare
///     name: sender_share
///     properties:
///       name: tf-test-resource-share
///       allowExternalPrincipals: true
///       tags:
///         Name: tf-test-resource-share
///   senderInvite:
///     type: aws:ram:PrincipalAssociation
///     name: sender_invite
///     properties:
///       principal: ${receiver.accountId}
///       resourceShareArn: ${senderShare.arn}
///   receiverAccept:
///     type: aws:ram:ResourceShareAccepter
///     name: receiver_accept
///     properties:
///       shareArn: ${senderInvite.resourceShareArn}
/// variables:
///   receiver:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import resource share accepters using the resource share ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ram/resourceShareAccepter:ResourceShareAccepter example arn:aws:ram:us-east-1:123456789012:resource-share/c4b56393-e8d9-89d9-6dc9-883752de4767
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_share_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceShareAccepterArgs {
        /// The ARN of the resource share.
        #[builder(into)]
        pub share_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceShareAccepterResult {
        /// The ARN of the resource share invitation.
        pub invitation_arn: pulumi_gestalt_rust::Output<String>,
        /// The account ID of the receiver account which accepts the invitation.
        pub receiver_account_id: pulumi_gestalt_rust::Output<String>,
        /// A list of the resource ARNs shared via the resource share.
        pub resources: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The account ID of the sender account which submits the invitation.
        pub sender_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the resource share.
        pub share_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the resource share as displayed in the console.
        pub share_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource share.
        pub share_name: pulumi_gestalt_rust::Output<String>,
        /// The status of the resource share (ACTIVE, PENDING, FAILED, DELETING, DELETED).
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ResourceShareAccepterArgs,
    ) -> ResourceShareAccepterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let share_arn_binding_1 = args.share_arn.get_output(context);
        let share_arn_binding = share_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ram/resourceShareAccepter:ResourceShareAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "shareArn".into(),
                    value: &share_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResourceShareAccepterResult {
            invitation_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("invitationArn"),
            ),
            receiver_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("receiverAccountId"),
            ),
            resources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resources"),
            ),
            sender_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("senderAccountId"),
            ),
            share_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareArn"),
            ),
            share_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareId"),
            ),
            share_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareName"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
