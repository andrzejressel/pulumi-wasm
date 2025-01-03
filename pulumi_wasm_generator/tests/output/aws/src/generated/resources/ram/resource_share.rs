/// Manages a Resource Access Manager (RAM) Resource Share. To associate principals with the share, see the `aws.ram.PrincipalAssociation` resource. To associate resources with the share, see the `aws.ram.ResourceAssociation` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ram:ResourceShare
///     properties:
///       name: example
///       allowExternalPrincipals: true
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import resource shares using the `arn` of the resource share. For example:
///
/// ```sh
/// $ pulumi import aws:ram/resourceShare:ResourceShare example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12
/// ```
pub mod resource_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceShareArgs {
        /// Indicates whether principals outside your organization can be associated with a resource share.
        #[builder(into, default)]
        pub allow_external_principals: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource share.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
        #[builder(into, default)]
        pub permission_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceShareResult {
        /// Indicates whether principals outside your organization can be associated with a resource share.
        pub allow_external_principals: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the resource share.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the resource share.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
        pub permission_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceShareArgs) -> ResourceShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_external_principals_binding = args
            .allow_external_principals
            .get_inner();
        let name_binding = args.name.get_inner();
        let permission_arns_binding = args.permission_arns.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ram/resourceShare:ResourceShare".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowExternalPrincipals".into(),
                    value: &allow_external_principals_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissionArns".into(),
                    value: &permission_arns_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowExternalPrincipals".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissionArns".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceShareResult {
            allow_external_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowExternalPrincipals").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permission_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionArns").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
