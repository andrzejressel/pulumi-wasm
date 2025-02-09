/// Provides a Step Function Activity resource
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sfnActivity = activity::create(
///         "sfnActivity",
///         ActivityArgs::builder().name("my-activity").build_struct(),
///     );
/// }
/// ```
///
/// ### Encryption
///
/// > *NOTE:* See the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the [AWS Step Functions Developer Guide](https://docs.aws.amazon.com/step-functions/latest/dg/welcome.html) for more information about enabling encryption of data using a customer-managed key for Step Functions State Machines data.
///
/// ```yaml
/// resources:
///   sfnActivity:
///     type: aws:sfn:Activity
///     name: sfn_activity
///     properties:
///       name: my-activity
///       encryptionConfiguration:
///         kmsKeyId: ${kmsKeyForSfn.arn}
///         type: CUSTOMER_MANAGED_KMS_KEY
///         kmsDataKeyReusePeriodSeconds: 900
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import activities using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sfn/activity:Activity foo arn:aws:states:eu-west-1:123456789098:activity:bar
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod activity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActivityArgs {
        /// Defines what encryption configuration is used to encrypt data in the Activity. For more information see the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the AWS Step Functions User Guide.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::sfn::ActivityEncryptionConfiguration>,
        >,
        /// The name of the activity to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActivityResult {
        /// The date the activity was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Defines what encryption configuration is used to encrypt data in the Activity. For more information see the section [Data at rest encyption](https://docs.aws.amazon.com/step-functions/latest/dg/encryption-at-rest.html) in the AWS Step Functions User Guide.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            super::super::types::sfn::ActivityEncryptionConfiguration,
        >,
        /// The name of the activity to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ActivityArgs,
    ) -> ActivityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let encryption_configuration_binding_1 = args
            .encryption_configuration
            .get_output(context);
        let encryption_configuration_binding = encryption_configuration_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sfn/activity:Activity".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ActivityResult {
            creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationDate"),
            ),
            encryption_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfiguration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
