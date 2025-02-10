/// Provides an alias for a KMS customer master key. AWS Console enforces 1-to-1 mapping between aliases & keys,
/// but API (hence this provider too) allows you to create as many aliases as
/// the [account limits](http://docs.aws.amazon.com/kms/latest/developerguide/limits.html) allow you.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let a = key::create("a", KeyArgs::builder().build_struct());
///     let aAlias = alias::create(
///         "aAlias",
///         AliasArgs::builder()
///             .name("alias/my-key-alias")
///             .target_key_id("${a.keyId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS aliases using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/alias:Alias a alias/my-key-alias
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// The display name of the alias. The name must start with the word "alias" followed by a forward slash (alias/)
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates an unique alias beginning with the specified prefix.
        /// The name must start with the word "alias" followed by a forward slash (alias/).  Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier for the key for which the alias is for, can be either an ARN or key_id.
        #[builder(into)]
        pub target_key_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) of the key alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The display name of the alias. The name must start with the word "alias" followed by a forward slash (alias/)
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates an unique alias beginning with the specified prefix.
        /// The name must start with the word "alias" followed by a forward slash (alias/).  Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the target key identifier.
        pub target_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the key for which the alias is for, can be either an ARN or key_id.
        pub target_key_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AliasArgs,
    ) -> AliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let target_key_id_binding = args.target_key_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/alias:Alias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetKeyId".into(),
                    value: target_key_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AliasResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            target_key_arn: o.get_field("targetKeyArn"),
            target_key_id: o.get_field("targetKeyId"),
        }
    }
}
