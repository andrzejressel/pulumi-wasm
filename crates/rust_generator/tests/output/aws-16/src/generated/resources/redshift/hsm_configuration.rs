/// Creates an HSM configuration that contains the information required by an Amazon Redshift cluster to store and use database encryption keys in a Hardware Security Module (HSM).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hsm_configuration::create(
///         "example",
///         HsmConfigurationArgs::builder()
///             .description("example")
///             .hsm_configuration_identifier("example")
///             .hsm_ip_address("10.0.0.1")
///             .hsm_partition_name("aws")
///             .hsm_partition_password("example")
///             .hsm_server_public_certificate("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift HSM Client Certificates using `hsm_configuration_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/hsmConfiguration:HsmConfiguration example example
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod hsm_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HsmConfigurationArgs {
        /// A text description of the HSM configuration to be created.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identifier to be assigned to the new Amazon Redshift HSM configuration.
        #[builder(into)]
        pub hsm_configuration_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IP address that the Amazon Redshift cluster must use to access the HSM.
        #[builder(into)]
        pub hsm_ip_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.
        #[builder(into)]
        pub hsm_partition_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password required to access the HSM partition.
        #[builder(into)]
        pub hsm_partition_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.
        #[builder(into)]
        pub hsm_server_public_certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HsmConfigurationResult {
        /// Amazon Resource Name (ARN) of the Hsm Client Certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A text description of the HSM configuration to be created.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The identifier to be assigned to the new Amazon Redshift HSM configuration.
        pub hsm_configuration_identifier: pulumi_gestalt_rust::Output<String>,
        /// The IP address that the Amazon Redshift cluster must use to access the HSM.
        pub hsm_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The name of the partition in the HSM where the Amazon Redshift clusters will store their database encryption keys.
        pub hsm_partition_name: pulumi_gestalt_rust::Output<String>,
        /// The password required to access the HSM partition.
        pub hsm_partition_password: pulumi_gestalt_rust::Output<String>,
        /// The HSMs public certificate file. When using Cloud HSM, the file name is server.pem.
        pub hsm_server_public_certificate: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: HsmConfigurationArgs,
    ) -> HsmConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let hsm_configuration_identifier_binding = args
            .hsm_configuration_identifier
            .get_output(context)
            .get_inner();
        let hsm_ip_address_binding = args.hsm_ip_address.get_output(context).get_inner();
        let hsm_partition_name_binding = args
            .hsm_partition_name
            .get_output(context)
            .get_inner();
        let hsm_partition_password_binding = args
            .hsm_partition_password
            .get_output(context)
            .get_inner();
        let hsm_server_public_certificate_binding = args
            .hsm_server_public_certificate
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/hsmConfiguration:HsmConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hsmConfigurationIdentifier".into(),
                    value: &hsm_configuration_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "hsmIpAddress".into(),
                    value: &hsm_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "hsmPartitionName".into(),
                    value: &hsm_partition_name_binding,
                },
                register_interface::ObjectField {
                    name: "hsmPartitionPassword".into(),
                    value: &hsm_partition_password_binding,
                },
                register_interface::ObjectField {
                    name: "hsmServerPublicCertificate".into(),
                    value: &hsm_server_public_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HsmConfigurationResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            hsm_configuration_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmConfigurationIdentifier"),
            ),
            hsm_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmIpAddress"),
            ),
            hsm_partition_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmPartitionName"),
            ),
            hsm_partition_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmPartitionPassword"),
            ),
            hsm_server_public_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hsmServerPublicCertificate"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
