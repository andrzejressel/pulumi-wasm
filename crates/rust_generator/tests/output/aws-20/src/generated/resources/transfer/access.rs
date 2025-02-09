/// Provides a AWS Transfer Access resource.
///
/// > **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.
///
/// ## Example Usage
///
/// ### Basic S3
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access::create(
///         "example",
///         AccessArgs::builder()
///             .external_id("S-1-1-12-1234567890-123456789-1234567890-1234")
///             .home_directory("/${exampleAwsS3Bucket.id}/")
///             .role("${exampleAwsIamRole.arn}")
///             .server_id("${exampleAwsTransferServer.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic EFS
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = access::create(
///         "test",
///         AccessArgs::builder()
///             .external_id("S-1-1-12-1234567890-123456789-1234567890-1234")
///             .home_directory("/${testAwsEfsFileSystem.id}/")
///             .posix_profile(
///                 AccessPosixProfile::builder().gid(1000).uid(1000).build_struct(),
///             )
///             .role("${testAwsIamRole.arn}")
///             .server_id("${testAwsTransferServer.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer Accesses using the `server_id` and `external_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/access:Access example s-12345678/S-1-1-12-1234567890-123456789-1234567890-1234
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessArgs {
        /// The SID of a group in the directory connected to the Transfer Server (e.g., `S-1-1-12-1234567890-123456789-1234567890-1234`)
        #[builder(into)]
        pub external_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
        #[builder(into, default)]
        pub home_directory: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
        #[builder(into, default)]
        pub home_directory_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::transfer::AccessHomeDirectoryMapping>>,
        >,
        /// The type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
        #[builder(into, default)]
        pub home_directory_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
        #[builder(into, default)]
        pub posix_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::transfer::AccessPosixProfile>,
        >,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to controls your user’s access to your Amazon S3 bucket.
        #[builder(into, default)]
        pub role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        #[builder(into)]
        pub server_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessResult {
        /// The SID of a group in the directory connected to the Transfer Server (e.g., `S-1-1-12-1234567890-123456789-1234567890-1234`)
        pub external_id: pulumi_gestalt_rust::Output<String>,
        /// The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
        pub home_directory: pulumi_gestalt_rust::Output<Option<String>>,
        /// Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
        pub home_directory_mappings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::transfer::AccessHomeDirectoryMapping>>,
        >,
        /// The type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
        pub home_directory_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
        pub policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
        pub posix_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::transfer::AccessPosixProfile>,
        >,
        /// Amazon Resource Name (ARN) of an IAM role that allows the service to controls your user’s access to your Amazon S3 bucket.
        pub role: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Server ID of the Transfer Server (e.g., `s-12345678`)
        pub server_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessArgs,
    ) -> AccessResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let external_id_binding = args.external_id.get_output(context);
        let home_directory_binding = args.home_directory.get_output(context);
        let home_directory_mappings_binding = args
            .home_directory_mappings
            .get_output(context);
        let home_directory_type_binding = args.home_directory_type.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let posix_profile_binding = args.posix_profile.get_output(context);
        let role_binding = args.role.get_output(context);
        let server_id_binding = args.server_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:transfer/access:Access".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "externalId".into(),
                    value: external_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectory".into(),
                    value: home_directory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectoryMappings".into(),
                    value: home_directory_mappings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "homeDirectoryType".into(),
                    value: home_directory_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "posixProfile".into(),
                    value: posix_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverId".into(),
                    value: server_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessResult {
            external_id: o.get_field("externalId"),
            home_directory: o.get_field("homeDirectory"),
            home_directory_mappings: o.get_field("homeDirectoryMappings"),
            home_directory_type: o.get_field("homeDirectoryType"),
            policy: o.get_field("policy"),
            posix_profile: o.get_field("posixProfile"),
            role: o.get_field("role"),
            server_id: o.get_field("serverId"),
        }
    }
}
