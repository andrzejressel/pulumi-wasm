/// Provides an ElastiCache user resource.
///
/// > **Note:** All arguments including the username and passwords will be stored in the raw state as plain-text.
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Access permissions string used for this user. See [Specifying Permissions Using an Access String](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html#Access-string) for more details.
        #[builder(into)]
        pub access_string: pulumi_wasm_rust::Output<String>,
        /// Denotes the user's authentication properties. Detailed below.
        #[builder(into, default)]
        pub authentication_mode: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticache::UserAuthenticationMode>,
        >,
        /// The current supported value is `REDIS`.
        #[builder(into)]
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Indicates a password is not required for this user.
        #[builder(into, default)]
        pub no_password_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Passwords used for this user. You can create up to two passwords for each user.
        #[builder(into, default)]
        pub passwords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of tags to be added to this resource. A tag is a key-value pair.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the user.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// The username of the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Access permissions string used for this user. See [Specifying Permissions Using an Access String](https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html#Access-string) for more details.
        pub access_string: pulumi_wasm_rust::Output<String>,
        /// The ARN of the created ElastiCache User.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Denotes the user's authentication properties. Detailed below.
        pub authentication_mode: pulumi_wasm_rust::Output<
            super::super::types::elasticache::UserAuthenticationMode,
        >,
        /// The current supported value is `REDIS`.
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Indicates a password is not required for this user.
        pub no_password_required: pulumi_wasm_rust::Output<Option<bool>>,
        /// Passwords used for this user. You can create up to two passwords for each user.
        pub passwords: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of tags to be added to this resource. A tag is a key-value pair.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the user.
        pub user_id: pulumi_wasm_rust::Output<String>,
        /// The username of the user.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserArgs) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_string_binding = args.access_string.get_inner();
        let authentication_mode_binding = args.authentication_mode.get_inner();
        let engine_binding = args.engine.get_inner();
        let no_password_required_binding = args.no_password_required.get_inner();
        let passwords_binding = args.passwords.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_id_binding = args.user_id.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticache/user:User".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessString".into(),
                    value: &access_string_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "noPasswordRequired".into(),
                    value: &no_password_required_binding,
                },
                register_interface::ObjectField {
                    name: "passwords".into(),
                    value: &passwords_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessString".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authenticationMode".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "noPasswordRequired".into(),
                },
                register_interface::ResultField {
                    name: "passwords".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userId".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserResult {
            access_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessString").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationMode").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            no_password_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noPasswordRequired").unwrap(),
            ),
            passwords: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("passwords").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userId").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}