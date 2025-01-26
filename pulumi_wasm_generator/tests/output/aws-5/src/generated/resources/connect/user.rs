/// Provides an Amazon Connect User resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .identity_info(
///                 UserIdentityInfo::builder()
///                     .firstName("example")
///                     .lastName("example2")
///                     .build_struct(),
///             )
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .name("example")
///             .password("Password123")
///             .phone_config(
///                 UserPhoneConfig::builder()
///                     .afterContactWorkTimeLimit(0)
///                     .phoneType("SOFT_PHONE")
///                     .build_struct(),
///             )
///             .routing_profile_id("${exampleAwsConnectRoutingProfile.routingProfileId}")
///             .security_profile_ids(
///                 vec!["${exampleAwsConnectSecurityProfile.securityProfileId}",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With hierarchy_group_id
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .hierarchy_group_id(
///                 "${exampleAwsConnectUserHierarchyGroup.hierarchyGroupId}",
///             )
///             .identity_info(
///                 UserIdentityInfo::builder()
///                     .firstName("example")
///                     .lastName("example2")
///                     .build_struct(),
///             )
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .name("example")
///             .password("Password123")
///             .phone_config(
///                 UserPhoneConfig::builder()
///                     .afterContactWorkTimeLimit(0)
///                     .phoneType("SOFT_PHONE")
///                     .build_struct(),
///             )
///             .routing_profile_id("${exampleAwsConnectRoutingProfile.routingProfileId}")
///             .security_profile_ids(
///                 vec!["${exampleAwsConnectSecurityProfile.securityProfileId}",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With identity_info filled
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .identity_info(
///                 UserIdentityInfo::builder()
///                     .email("example@example.com")
///                     .firstName("example")
///                     .lastName("example2")
///                     .build_struct(),
///             )
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .name("example")
///             .password("Password123")
///             .phone_config(
///                 UserPhoneConfig::builder()
///                     .afterContactWorkTimeLimit(0)
///                     .phoneType("SOFT_PHONE")
///                     .build_struct(),
///             )
///             .routing_profile_id("${exampleAwsConnectRoutingProfile.routingProfileId}")
///             .security_profile_ids(
///                 vec!["${exampleAwsConnectSecurityProfile.securityProfileId}",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With phone_config phone type as desk phone
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .name("example")
///             .password("Password123")
///             .phone_config(
///                 UserPhoneConfig::builder()
///                     .afterContactWorkTimeLimit(0)
///                     .phoneType("SOFT_PHONE")
///                     .build_struct(),
///             )
///             .routing_profile_id("${exampleAwsConnectRoutingProfile.routingProfileId}")
///             .security_profile_ids(
///                 vec!["${exampleAwsConnectSecurityProfile.securityProfileId}",],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### With multiple Security profile ids specified in security_profile_ids
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .instance_id("${exampleAwsConnectInstance.id}")
///             .name("example")
///             .password("Password123")
///             .phone_config(
///                 UserPhoneConfig::builder()
///                     .afterContactWorkTimeLimit(0)
///                     .autoAccept(false)
///                     .deskPhoneNumber("+112345678912")
///                     .phoneType("DESK_PHONE")
///                     .build_struct(),
///             )
///             .routing_profile_id("${exampleAwsConnectRoutingProfile.routingProfileId}")
///             .security_profile_ids(
///                 vec![
///                     "${exampleAwsConnectSecurityProfile.securityProfileId}",
///                     "${example2.securityProfileId}",
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Users using the `instance_id` and `user_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/user:User example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory. This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.
        #[builder(into, default)]
        pub directory_user_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The identifier of the hierarchy group for the user.
        #[builder(into, default)]
        pub hierarchy_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block that contains information about the identity of the user. Documented below.
        #[builder(into, default)]
        pub identity_info: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::connect::UserIdentityInfo>,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from `[a-zA-Z0-9_-.\@]+`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A block that contains information about the phone settings for the user. Documented below.
        #[builder(into)]
        pub phone_config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::connect::UserPhoneConfig,
        >,
        /// The identifier of the routing profile for the user.
        #[builder(into)]
        pub routing_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of identifiers for the security profiles for the user. Specify a minimum of 1 and maximum of 10 security profile ids. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
        #[builder(into)]
        pub security_profile_ids: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// Tags to apply to the user. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// The Amazon Resource Name (ARN) of the user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the user account in the directory used for identity management. If Amazon Connect cannot access the directory, you can specify this identifier to authenticate users. If you include the identifier, we assume that Amazon Connect cannot access the directory. Otherwise, the identity information is used to authenticate users from your directory. This parameter is required if you are using an existing directory for identity management in Amazon Connect when Amazon Connect cannot access your directory to authenticate users. If you are using SAML for identity management and include this parameter, an error is returned.
        pub directory_user_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the hierarchy group for the user.
        pub hierarchy_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that contains information about the identity of the user. Documented below.
        pub identity_info: pulumi_wasm_rust::Output<
            Option<super::super::types::connect::UserIdentityInfo>,
        >,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The user name for the account. For instances not using SAML for identity management, the user name can include up to 20 characters. If you are using SAML for identity management, the user name can include up to 64 characters from `[a-zA-Z0-9_-.\@]+`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password for the user account. A password is required if you are using Amazon Connect for identity management. Otherwise, it is an error to include a password.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// A block that contains information about the phone settings for the user. Documented below.
        pub phone_config: pulumi_wasm_rust::Output<
            super::super::types::connect::UserPhoneConfig,
        >,
        /// The identifier of the routing profile for the user.
        pub routing_profile_id: pulumi_wasm_rust::Output<String>,
        /// A list of identifiers for the security profiles for the user. Specify a minimum of 1 and maximum of 10 security profile ids. For more information, see [Best Practices for Security Profiles](https://docs.aws.amazon.com/connect/latest/adminguide/security-profile-best-practices.html) in the Amazon Connect Administrator Guide.
        pub security_profile_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Tags to apply to the user. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The identifier for the user.
        pub user_id: pulumi_wasm_rust::Output<String>,
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
        let directory_user_id_binding = args
            .directory_user_id
            .get_output(context)
            .get_inner();
        let hierarchy_group_id_binding = args
            .hierarchy_group_id
            .get_output(context)
            .get_inner();
        let identity_info_binding = args.identity_info.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let phone_config_binding = args.phone_config.get_output(context).get_inner();
        let routing_profile_id_binding = args
            .routing_profile_id
            .get_output(context)
            .get_inner();
        let security_profile_ids_binding = args
            .security_profile_ids
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/user:User".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryUserId".into(),
                    value: &directory_user_id_binding,
                },
                register_interface::ObjectField {
                    name: "hierarchyGroupId".into(),
                    value: &hierarchy_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "identityInfo".into(),
                    value: &identity_info_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "phoneConfig".into(),
                    value: &phone_config_binding,
                },
                register_interface::ObjectField {
                    name: "routingProfileId".into(),
                    value: &routing_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "securityProfileIds".into(),
                    value: &security_profile_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "directoryUserId".into(),
                },
                register_interface::ResultField {
                    name: "hierarchyGroupId".into(),
                },
                register_interface::ResultField {
                    name: "identityInfo".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "phoneConfig".into(),
                },
                register_interface::ResultField {
                    name: "routingProfileId".into(),
                },
                register_interface::ResultField {
                    name: "securityProfileIds".into(),
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            directory_user_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryUserId").unwrap(),
            ),
            hierarchy_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hierarchyGroupId").unwrap(),
            ),
            identity_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityInfo").unwrap(),
            ),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            phone_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phoneConfig").unwrap(),
            ),
            routing_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingProfileId").unwrap(),
            ),
            security_profile_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityProfileIds").unwrap(),
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
        }
    }
}
