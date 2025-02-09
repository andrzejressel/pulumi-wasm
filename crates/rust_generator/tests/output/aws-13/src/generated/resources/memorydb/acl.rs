/// Provides a MemoryDB ACL.
///
/// More information about users and ACL-s can be found in the [MemoryDB User Guide](https://docs.aws.amazon.com/memorydb/latest/devguide/clusters.acls.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = acl::create(
///         "example",
///         AclArgs::builder()
///             .name("my-acl")
///             .user_names(vec!["my-user-1", "my-user-2",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an ACL using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:memorydb/acl:Acl example my-acl
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod acl {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AclArgs {
        /// Name of the ACL. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Set of MemoryDB user names to be included in this ACL.
        #[builder(into, default)]
        pub user_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AclResult {
        /// The ARN of the ACL.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The minimum engine version supported by the ACL.
        pub minimum_engine_version: pulumi_gestalt_rust::Output<String>,
        /// Name of the ACL. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of MemoryDB user names to be included in this ACL.
        pub user_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AclArgs,
    ) -> AclResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let user_names_binding_1 = args.user_names.get_output(context);
        let user_names_binding = user_names_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:memorydb/acl:Acl".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userNames".into(),
                    value: &user_names_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AclResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            minimum_engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumEngineVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            user_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userNames"),
            ),
        }
    }
}
