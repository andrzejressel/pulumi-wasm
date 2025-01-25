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
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// Access permissions string used for this user.
        #[builder(into)]
        pub access_string: pulumi_wasm_rust::InputOrOutput<String>,
        /// Denotes the user's authentication properties. Detailed below.
        #[builder(into)]
        pub authentication_mode: pulumi_wasm_rust::InputOrOutput<
            super::super::types::memorydb::UserAuthenticationMode,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the MemoryDB user. Up to 40 characters.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Access permissions string used for this user.
        pub access_string: pulumi_wasm_rust::Output<String>,
        /// ARN of the user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Denotes the user's authentication properties. Detailed below.
        pub authentication_mode: pulumi_wasm_rust::Output<
            super::super::types::memorydb::UserAuthenticationMode,
        >,
        /// Minimum engine version supported for the user.
        pub minimum_engine_version: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name of the MemoryDB user. Up to 40 characters.
        ///
        /// The following arguments are optional:
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UserArgs,
    ) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_string_binding = args.access_string.get_output(context).get_inner();
        let authentication_mode_binding = args
            .authentication_mode
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:memorydb/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "tags".into(),
                    value: &tags_binding,
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
                    name: "minimumEngineVersion".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
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
            minimum_engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumEngineVersion").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
        }
    }
}
