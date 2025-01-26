/// Provides a Lightsail Instance. Amazon Lightsail is a service to provide easy virtual private servers
/// with custom software already setup. See [What is Amazon Lightsail?](https://lightsail.aws.amazon.com/ls/docs/getting-started/article/what-is-amazon-lightsail)
/// for more information.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   # Create a new GitLab Lightsail Instance
///   gitlabTest:
///     type: aws:lightsail:Instance
///     name: gitlab_test
///     properties:
///       name: custom_gitlab
///       availabilityZone: us-east-1b
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///       keyPairName: some_key_name
///       tags:
///         foo: bar
/// ```
///
/// ### Example With User Data
///
/// Lightsail user data is handled differently than ec2 user data. Lightsail user data only accepts a single lined string. The below example shows installing apache and creating the index page.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let custom = instance::create(
///         "custom",
///         InstanceArgs::builder()
///             .availability_zone("us-east-1b")
///             .blueprint_id("amazon_linux_2")
///             .bundle_id("nano_3_0")
///             .name("custom")
///             .user_data(
///                 "sudo yum install -y httpd && sudo systemctl start httpd && sudo systemctl enable httpd && echo '<h1>Deployed via Pulumi</h1>' | sudo tee /var/www/html/index.html",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enable Auto Snapshots
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lightsail:Instance
///     properties:
///       name: custom_instance
///       availabilityZone: us-east-1b
///       blueprintId: amazon_linux_2
///       bundleId: nano_3_0
///       addOn:
///         type: AutoSnapshot
///         snapshotTime: 06:00
///         status: Enabled
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Instances using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/instance:Instance gitlab_test 'custom_gitlab'
/// ```
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The add on configuration for the instance. Detailed below.
        #[builder(into, default)]
        pub add_on: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lightsail::InstanceAddOn>,
        >,
        /// The Availability Zone in which to create your instance. A
        /// list of available zones can be obtained using the AWS CLI command:
        /// [`aws lightsail get-regions --include-availability-zones`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-regions.html).
        #[builder(into)]
        pub availability_zone: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID for a virtual private server image. A list of available
        /// blueprint IDs can be obtained using the AWS CLI command:
        /// [`aws lightsail get-blueprints`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-blueprints.html).
        #[builder(into)]
        pub blueprint_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The bundle of specification information. A list of available
        /// bundle IDs can be obtained using the AWS CLI command:
        /// [`aws lightsail get-bundles`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-bundles.html).
        #[builder(into)]
        pub bundle_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The IP address type of the Lightsail Instance. Valid Values: `dualstack` | `ipv4`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of your key pair. Created in the
        /// Lightsail console (cannot use `aws.ec2.KeyPair` at this time)
        #[builder(into, default)]
        pub key_pair_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Lightsail Instance. Names must be unique within each AWS Region in your Lightsail account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Single lined launch script as a string to configure server with additional user data
        #[builder(into, default)]
        pub user_data: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// The add on configuration for the instance. Detailed below.
        pub add_on: pulumi_wasm_rust::Output<
            Option<super::super::types::lightsail::InstanceAddOn>,
        >,
        /// The ARN of the Lightsail instance (matches `id`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone in which to create your instance. A
        /// list of available zones can be obtained using the AWS CLI command:
        /// [`aws lightsail get-regions --include-availability-zones`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-regions.html).
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// The ID for a virtual private server image. A list of available
        /// blueprint IDs can be obtained using the AWS CLI command:
        /// [`aws lightsail get-blueprints`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-blueprints.html).
        pub blueprint_id: pulumi_wasm_rust::Output<String>,
        /// The bundle of specification information. A list of available
        /// bundle IDs can be obtained using the AWS CLI command:
        /// [`aws lightsail get-bundles`](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/lightsail/get-bundles.html).
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The number of vCPUs the instance has.
        pub cpu_count: pulumi_wasm_rust::Output<i32>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The IP address type of the Lightsail Instance. Valid Values: `dualstack` | `ipv4`.
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// List of IPv6 addresses for the Lightsail instance.
        pub ipv6_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// A Boolean value indicating whether this instance has a static IP assigned to it.
        pub is_static_ip: pulumi_wasm_rust::Output<bool>,
        /// The name of your key pair. Created in the
        /// Lightsail console (cannot use `aws.ec2.KeyPair` at this time)
        pub key_pair_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Lightsail Instance. Names must be unique within each AWS Region in your Lightsail account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The private IP address of the instance.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The public IP address of the instance.
        pub public_ip_address: pulumi_wasm_rust::Output<String>,
        /// The amount of RAM in GB on the instance (e.g., 1.0).
        pub ram_size: pulumi_wasm_rust::Output<f64>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Single lined launch script as a string to configure server with additional user data
        pub user_data: pulumi_wasm_rust::Output<Option<String>>,
        /// The user name for connecting to the instance (e.g., ec2-user).
        pub username: pulumi_wasm_rust::Output<String>,
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
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let add_on_binding = args.add_on.get_output(context).get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let blueprint_id_binding = args.blueprint_id.get_output(context).get_inner();
        let bundle_id_binding = args.bundle_id.get_output(context).get_inner();
        let ip_address_type_binding = args
            .ip_address_type
            .get_output(context)
            .get_inner();
        let key_pair_name_binding = args.key_pair_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_data_binding = args.user_data.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addOn".into(),
                    value: &add_on_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "blueprintId".into(),
                    value: &blueprint_id_binding,
                },
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "keyPairName".into(),
                    value: &key_pair_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userData".into(),
                    value: &user_data_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addOn".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "blueprintId".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "cpuCount".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Addresses".into(),
                },
                register_interface::ResultField {
                    name: "isStaticIp".into(),
                },
                register_interface::ResultField {
                    name: "keyPairName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "publicIpAddress".into(),
                },
                register_interface::ResultField {
                    name: "ramSize".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userData".into(),
                },
                register_interface::ResultField {
                    name: "username".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            add_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addOn").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            blueprint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blueprintId").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            cpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuCount").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            ipv6_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Addresses").unwrap(),
            ),
            is_static_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isStaticIp").unwrap(),
            ),
            key_pair_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyPairName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpAddress").unwrap(),
            ),
            public_ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicIpAddress").unwrap(),
            ),
            ram_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ramSize").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userData").unwrap(),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("username").unwrap(),
            ),
        }
    }
}
