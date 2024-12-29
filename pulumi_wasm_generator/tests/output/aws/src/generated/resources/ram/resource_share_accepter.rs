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
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import resource share accepters using the resource share ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ram/resourceShareAccepter:ResourceShareAccepter example arn:aws:ram:us-east-1:123456789012:resource-share/c4b56393-e8d9-89d9-6dc9-883752de4767
/// ```
pub mod resource_share_accepter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceShareAccepterArgs {
        /// The ARN of the resource share.
        #[builder(into)]
        pub share_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceShareAccepterResult {
        /// The ARN of the resource share invitation.
        pub invitation_arn: pulumi_wasm_rust::Output<String>,
        /// The account ID of the receiver account which accepts the invitation.
        pub receiver_account_id: pulumi_wasm_rust::Output<String>,
        /// A list of the resource ARNs shared via the resource share.
        pub resources: pulumi_wasm_rust::Output<Vec<String>>,
        /// The account ID of the sender account which submits the invitation.
        pub sender_account_id: pulumi_wasm_rust::Output<String>,
        /// The ARN of the resource share.
        pub share_arn: pulumi_wasm_rust::Output<String>,
        /// The ID of the resource share as displayed in the console.
        pub share_id: pulumi_wasm_rust::Output<String>,
        /// The name of the resource share.
        pub share_name: pulumi_wasm_rust::Output<String>,
        /// The status of the resource share (ACTIVE, PENDING, FAILED, DELETING, DELETED).
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ResourceShareAccepterArgs,
    ) -> ResourceShareAccepterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let share_arn_binding = args.share_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ram/resourceShareAccepter:ResourceShareAccepter".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "shareArn".into(),
                    value: &share_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "invitationArn".into(),
                },
                register_interface::ResultField {
                    name: "receiverAccountId".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "senderAccountId".into(),
                },
                register_interface::ResultField {
                    name: "shareArn".into(),
                },
                register_interface::ResultField {
                    name: "shareId".into(),
                },
                register_interface::ResultField {
                    name: "shareName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceShareAccepterResult {
            invitation_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("invitationArn").unwrap(),
            ),
            receiver_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("receiverAccountId").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            sender_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("senderAccountId").unwrap(),
            ),
            share_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareArn").unwrap(),
            ),
            share_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareId").unwrap(),
            ),
            share_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
