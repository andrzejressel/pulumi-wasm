/// Provides an SSM Parameter resource.
///
/// > **Note:** `overwrite` also makes it possible to overwrite an existing SSM Parameter that's not created by the provider before. This argument has been deprecated and will be removed in v6.0.0 of the provider. For more information on how this affects the behavior of this resource, see this issue comment.
///
/// ## Example Usage
///
/// ### Basic example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = parameter::create(
///         "foo",
///         ParameterArgs::builder().name("foo").type_("String").value("bar").build_struct(),
///     );
/// }
/// ```
///
/// ### Encrypted string using default SSM KMS key
///
/// ```yaml
/// resources:
///   default:
///     type: aws:rds:Instance
///     properties:
///       allocatedStorage: 10
///       storageType: gp2
///       engine: mysql
///       engineVersion: 5.7.16
///       instanceClass: db.t2.micro
///       dbName: mydb
///       username: foo
///       password: ${databaseMasterPassword}
///       dbSubnetGroupName: my_database_subnet_group
///       parameterGroupName: default.mysql5.7
///   secret:
///     type: aws:ssm:Parameter
///     properties:
///       name: /production/database/password/master
///       description: The parameter description
///       type: SecureString
///       value: ${databaseMasterPassword}
///       tags:
///         environment: production
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Parameters using the parameter store `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/parameter:Parameter my_param /my_path/my_paramname
/// ```
pub mod parameter {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterArgs {
        /// Regular expression used to validate the parameter value.
        #[builder(into, default)]
        pub allowed_pattern: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the parameter.
        #[builder(into, default)]
        pub arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Data type of the parameter. Valid values: `text`, `aws:ssm:integration` and `aws:ec2:image` for AMI format, see the [Native parameter support for Amazon Machine Image IDs](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-ec2-aliases.html).
        #[builder(into, default)]
        pub data_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Description of the parameter.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Value of the parameter. **Use caution:** This value is _never_ marked as sensitive in the pulumi preview output. This argument is not valid with a `type` of `SecureString`.
        #[builder(into, default)]
        pub insecure_value: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// KMS key ID or ARN for encrypting a SecureString.
        #[builder(into, default)]
        pub key_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the parameter. If the name contains a path (e.g., any forward slashes (`/`)), it must be fully qualified with a leading forward slash (`/`). For additional requirements and constraints, see the [AWS SSM User Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Overwrite an existing parameter. If not specified, defaults to `false` if the resource has not been created by Pulumi to avoid overwrite of existing resource, and will default to `true` otherwise (Pulumi lifecycle rules should then be used to manage the update behavior).
        #[builder(into, default)]
        pub overwrite: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Parameter tier to assign to the parameter. If not specified, will use the default parameter tier for the region. Valid tiers are `Standard`, `Advanced`, and `Intelligent-Tiering`. Downgrading an `Advanced` tier parameter to `Standard` will recreate the resource. For more information on parameter tiers, see the [AWS SSM Parameter tier comparison and guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html).
        #[builder(into, default)]
        pub tier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Type of the parameter. Valid types are `String`, `StringList` and `SecureString`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// Value of the parameter. This value is always marked as sensitive in the pulumi preview output, regardless of `type`.
        ///
        /// > **NOTE:** `aws:ssm:integration` data_type parameters must be of the type `SecureString` and the name must start with the prefix `/d9d01087-4a3f-49e0-b0b4-d568d7826553/ssm/integrations/webhook/`. See [here](https://docs.aws.amazon.com/systems-manager/latest/userguide/creating-integrations.html) for information on the usage of `aws:ssm:integration` parameters.
        #[builder(into, default)]
        pub value: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ParameterResult {
        /// Regular expression used to validate the parameter value.
        pub allowed_pattern: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the parameter.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Data type of the parameter. Valid values: `text`, `aws:ssm:integration` and `aws:ec2:image` for AMI format, see the [Native parameter support for Amazon Machine Image IDs](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-ec2-aliases.html).
        pub data_type: pulumi_wasm_rust::Output<String>,
        /// Description of the parameter.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Value of the parameter. **Use caution:** This value is _never_ marked as sensitive in the pulumi preview output. This argument is not valid with a `type` of `SecureString`.
        pub insecure_value: pulumi_wasm_rust::Output<String>,
        /// KMS key ID or ARN for encrypting a SecureString.
        pub key_id: pulumi_wasm_rust::Output<String>,
        /// Name of the parameter. If the name contains a path (e.g., any forward slashes (`/`)), it must be fully qualified with a leading forward slash (`/`). For additional requirements and constraints, see the [AWS SSM User Guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-parameter-name-constraints.html).
        pub name: pulumi_wasm_rust::Output<String>,
        /// Overwrite an existing parameter. If not specified, defaults to `false` if the resource has not been created by Pulumi to avoid overwrite of existing resource, and will default to `true` otherwise (Pulumi lifecycle rules should then be used to manage the update behavior).
        pub overwrite: pulumi_wasm_rust::Output<Option<bool>>,
        /// Map of tags to assign to the object. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Parameter tier to assign to the parameter. If not specified, will use the default parameter tier for the region. Valid tiers are `Standard`, `Advanced`, and `Intelligent-Tiering`. Downgrading an `Advanced` tier parameter to `Standard` will recreate the resource. For more information on parameter tiers, see the [AWS SSM Parameter tier comparison and guide](https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-advanced-parameters.html).
        pub tier: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of the parameter. Valid types are `String`, `StringList` and `SecureString`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Value of the parameter. This value is always marked as sensitive in the pulumi preview output, regardless of `type`.
        ///
        /// > **NOTE:** `aws:ssm:integration` data_type parameters must be of the type `SecureString` and the name must start with the prefix `/d9d01087-4a3f-49e0-b0b4-d568d7826553/ssm/integrations/webhook/`. See [here](https://docs.aws.amazon.com/systems-manager/latest/userguide/creating-integrations.html) for information on the usage of `aws:ssm:integration` parameters.
        pub value: pulumi_wasm_rust::Output<String>,
        /// Version of the parameter.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ParameterArgs,
    ) -> ParameterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowed_pattern_binding = args
            .allowed_pattern
            .get_output(context)
            .get_inner();
        let arn_binding = args.arn.get_output(context).get_inner();
        let data_type_binding = args.data_type.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let insecure_value_binding = args.insecure_value.get_output(context).get_inner();
        let key_id_binding = args.key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let overwrite_binding = args.overwrite.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/parameter:Parameter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedPattern".into(),
                    value: &allowed_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "dataType".into(),
                    value: &data_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "insecureValue".into(),
                    value: &insecure_value_binding,
                },
                register_interface::ObjectField {
                    name: "keyId".into(),
                    value: &key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "overwrite".into(),
                    value: &overwrite_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ParameterResult {
            allowed_pattern: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedPattern"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            data_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataType"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            insecure_value: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("insecureValue"),
            ),
            key_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("keyId")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            overwrite: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("overwrite"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(o.extract_field("tier")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            value: pulumi_wasm_rust::__private::into_domain(o.extract_field("value")),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
