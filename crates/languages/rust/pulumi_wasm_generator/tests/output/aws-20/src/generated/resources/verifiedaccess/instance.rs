/// Resource for managing a Verified Access Instance.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:verifiedaccess:Instance
///     properties:
///       description: example
///       tags:
///         Name: example
/// ```
///
/// ### With `fips_enabled`
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance::create(
///         "example",
///         InstanceArgs::builder().fips_enabled(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Access Instances using the  `id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedaccess/instance:Instance example vai-1234567890abcdef0
/// ```
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// A description for the AWS Verified Access Instance.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enable or disable support for Federal Information Processing Standards (FIPS) on the AWS Verified Access Instance.
        #[builder(into, default)]
        pub fips_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The time that the Verified Access Instance was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// A description for the AWS Verified Access Instance.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable or disable support for Federal Information Processing Standards (FIPS) on the AWS Verified Access Instance.
        pub fips_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The time that the Verified Access Instance was last updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// One or more blocks of providing information about the AWS Verified Access Trust Providers. See verified_access_trust_providers below for details.One or more blocks
        pub verified_access_trust_providers: pulumi_wasm_rust::Output<
            Vec<super::super::types::verifiedaccess::InstanceVerifiedAccessTrustProvider>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let fips_enabled_binding = args.fips_enabled.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fipsEnabled".into(),
                    value: &fips_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            creation_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fips_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fipsEnabled"),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            verified_access_trust_providers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verifiedAccessTrustProviders"),
            ),
        }
    }
}
