/// Provides a resource to manage whether default EBS encryption is enabled for your AWS account in the current AWS region. To manage the default KMS key for the region, see the `aws.ebs.DefaultKmsKey` resource.
///
/// > **NOTE:** Removing this resource disables default EBS encryption.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = encryption_by_default::create(
///         "example",
///         EncryptionByDefaultArgs::builder().enabled(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the default EBS encryption state. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/encryptionByDefault:EncryptionByDefault example default
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod encryption_by_default {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EncryptionByDefaultArgs {
        /// Whether or not default EBS encryption is enabled. Valid values are `true` or `false`. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct EncryptionByDefaultResult {
        /// Whether or not default EBS encryption is enabled. Valid values are `true` or `false`. Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EncryptionByDefaultArgs,
    ) -> EncryptionByDefaultResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/encryptionByDefault:EncryptionByDefault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EncryptionByDefaultResult {
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
        }
    }
}
