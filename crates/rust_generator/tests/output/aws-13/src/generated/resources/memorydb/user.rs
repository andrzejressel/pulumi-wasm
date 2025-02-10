/// Provides a MemoryDB User.
///
/// More information about users and ACL-s can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/clusters.acls.html).
///
/// > **Note:** All arguments including the username and passwords will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: random:password
///     properties:
///       length: 16
///   exampleUser:
///     type: aws:memorydb:User
///     name: example
///     properties:
///       userName: my-user
///       accessString: on ~* &* +@all
///       authenticationMode:
///         type: password
///         passwords:
///           - ${example.result}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a user using the `user_name`. For example:
///
/// ```sh
/// $ pulumi import aws:memorydb/user:User example my-user
/// ```
/// The `passwords` are not available for imported resources, as this information cannot be read back from the MemoryDB API.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Access permissions string used for this user.
        #[builder(into)]
        pub access_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Denotes the user's authentication properties. Detailed below.
        #[builder(into)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::memorydb::UserAuthenticationMode,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the MemoryDB user. Up to 40 characters.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Access permissions string used for this user.
        pub access_string: pulumi_gestalt_rust::Output<String>,
        /// ARN of the user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Denotes the user's authentication properties. Detailed below.
        pub authentication_mode: pulumi_gestalt_rust::Output<
            super::super::types::memorydb::UserAuthenticationMode,
        >,
        /// Minimum engine version supported for the user.
        pub minimum_engine_version: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the MemoryDB user. Up to 40 characters.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let access_string_binding = args.access_string.get_output(context);
        let authentication_mode_binding = args.authentication_mode.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:memorydb/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessString".into(),
                    value: access_string_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: authentication_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            access_string: o.get_field("accessString"),
            arn: o.get_field("arn"),
            authentication_mode: o.get_field("authenticationMode"),
            minimum_engine_version: o.get_field("minimumEngineVersion"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_name: o.get_field("userName"),
        }
    }
}
