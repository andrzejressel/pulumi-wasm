/// Accepts a shared directory in a consumer account.
///
/// > **NOTE:** Destroying this resource removes the shared directory from the consumer account only.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = shared_directory::create(
///         "example",
///         SharedDirectoryArgs::builder()
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .notes("example")
///             .target(
///                 SharedDirectoryTarget::builder()
///                     .id("${receiver.accountId}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleSharedDirectoryAccepter = shared_directory_accepter::create(
///         "exampleSharedDirectoryAccepter",
///         SharedDirectoryAccepterArgs::builder()
///             .shared_directory_id("${example.sharedDirectoryId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Directory Service Shared Directories using the shared directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter example d-9267633ece
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod shared_directory_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterArgs {
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        #[builder(into)]
        pub shared_directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SharedDirectoryAccepterResult {
        /// Method used when sharing a directory (i.e., `ORGANIZATIONS` or `HANDSHAKE`).
        pub method: pulumi_gestalt_rust::Output<String>,
        /// Message sent by the directory owner to the directory consumer to help the directory consumer administrator determine whether to approve or reject the share invitation.
        pub notes: pulumi_gestalt_rust::Output<String>,
        /// Account identifier of the directory owner.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the Managed Microsoft AD directory from the perspective of the directory owner.
        pub owner_directory_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier of the directory that is stored in the directory consumer account that corresponds to the shared directory in the owner account.
        pub shared_directory_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SharedDirectoryAccepterArgs,
    ) -> SharedDirectoryAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let shared_directory_id_binding = args.shared_directory_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/sharedDirectoryAccepter:SharedDirectoryAccepter"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharedDirectoryId".into(),
                    value: &shared_directory_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SharedDirectoryAccepterResult {
            method: o.get_field("method"),
            notes: o.get_field("notes"),
            owner_account_id: o.get_field("ownerAccountId"),
            owner_directory_id: o.get_field("ownerDirectoryId"),
            shared_directory_id: o.get_field("sharedDirectoryId"),
        }
    }
}
