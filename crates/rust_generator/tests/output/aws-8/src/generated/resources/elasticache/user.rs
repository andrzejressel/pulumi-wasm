/// Provides an ElastiCache user resource.
///
/// > **Note:** All arguments including the username and passwords will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = user::create(
///         "test",
///         UserArgs::builder()
///             .access_string(
///                 "on ~app::* -@all +@read +@hash +@bitmap +@geo -setbit -bitfield -hset -hsetnx -hmset -hincrby -hincrbyfloat -hdel -bitop -geoadd -georadius -georadiusbymember",
///             )
///             .engine("REDIS")
///             .passwords(vec!["password123456789",])
///             .user_id("testUserId")
///             .user_name("testUserName")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ```yaml
/// resources:
///   test:
///     type: aws:elasticache:User
///     properties:
///       userId: testUserId
///       userName: testUserName
///       accessString: on ~* +@all
///       engine: REDIS
///       authenticationMode:
///         type: iam
/// ```
///
/// ```yaml
/// resources:
///   test:
///     type: aws:elasticache:User
///     properties:
///       userId: testUserId
///       userName: testUserName
///       accessString: on ~* +@all
///       engine: REDIS
///       authenticationMode:
///         type: password
///         passwords:
///           - password1
///           - password2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ElastiCache users using the `user_id`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticache/user:User my_user userId1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Access permissions string used for this user. See [Specifying Permissions Using an Access String](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html#Access-string) for more details.
        #[builder(into)]
        pub access_string: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Denotes the user's authentication properties. Detailed below.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::elasticache::UserAuthenticationMode>,
        >,
        /// The current supported value is `REDIS`.
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicates a password is not required for this user.
        #[builder(into, default)]
        pub no_password_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Passwords used for this user. You can create up to two passwords for each user.
        #[builder(into, default)]
        pub passwords: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of tags to be added to this resource. A tag is a key-value pair.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the user.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The username of the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Access permissions string used for this user. See [Specifying Permissions Using an Access String](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html#Access-string) for more details.
        pub access_string: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the created ElastiCache User.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Denotes the user's authentication properties. Detailed below.
        pub authentication_mode: pulumi_gestalt_rust::Output<
            super::super::types::elasticache::UserAuthenticationMode,
        >,
        /// The current supported value is `REDIS`.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Indicates a password is not required for this user.
        pub no_password_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Passwords used for this user. You can create up to two passwords for each user.
        pub passwords: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A list of tags to be added to this resource. A tag is a key-value pair.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the user.
        pub user_id: pulumi_gestalt_rust::Output<String>,
        /// The username of the user.
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
        let engine_binding = args.engine.get_output(context);
        let no_password_required_binding = args.no_password_required.get_output(context);
        let passwords_binding = args.passwords.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:elasticache/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accessString".into(),
                    value: &access_string_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: &engine_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noPasswordRequired".into(),
                    value: &no_password_required_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwords".into(),
                    value: &passwords_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: &user_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: &user_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserResult {
            access_string: o.get_field("accessString"),
            arn: o.get_field("arn"),
            authentication_mode: o.get_field("authenticationMode"),
            engine: o.get_field("engine"),
            no_password_required: o.get_field("noPasswordRequired"),
            passwords: o.get_field("passwords"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            user_id: o.get_field("userId"),
            user_name: o.get_field("userName"),
        }
    }
}
