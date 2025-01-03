/// Provides an SES configuration set resource.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .name("some-configuration-set-test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Require TLS Connections
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .delivery_options(
///                 ConfigurationSetDeliveryOptions::builder()
///                     .tlsPolicy("Require")
///                     .build_struct(),
///             )
///             .name("some-configuration-set-test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tracking Options
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = configuration_set::create(
///         "test",
///         ConfigurationSetArgs::builder()
///             .name("some-configuration-set-test")
///             .tracking_options(
///                 ConfigurationSetTrackingOptions::builder()
///                     .customRedirectDomain("sub.example.com")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES Configuration Sets using their `name`. For example:
///
/// ```sh
/// $ pulumi import aws:ses/configurationSet:ConfigurationSet test some-configuration-set-test
/// ```
pub mod configuration_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationSetArgs {
        /// Whether messages that use the configuration set are required to use TLS. See below.
        #[builder(into, default)]
        pub delivery_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ses::ConfigurationSetDeliveryOptions>,
        >,
        /// Name of the configuration set.
        ///
        /// The following argument is optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
        #[builder(into, default)]
        pub reputation_metrics_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether email sending is enabled or disabled for the configuration set. The default value is `true`.
        #[builder(into, default)]
        pub sending_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Domain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
        #[builder(into, default)]
        pub tracking_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ses::ConfigurationSetTrackingOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationSetResult {
        /// SES configuration set ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether messages that use the configuration set are required to use TLS. See below.
        pub delivery_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ses::ConfigurationSetDeliveryOptions>,
        >,
        /// Date and time at which the reputation metrics for the configuration set were last reset. Resetting these metrics is known as a fresh start.
        pub last_fresh_start: pulumi_wasm_rust::Output<String>,
        /// Name of the configuration set.
        ///
        /// The following argument is optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether or not Amazon SES publishes reputation metrics for the configuration set, such as bounce and complaint rates, to Amazon CloudWatch. The default value is `false`.
        pub reputation_metrics_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether email sending is enabled or disabled for the configuration set. The default value is `true`.
        pub sending_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Domain that is used to redirect email recipients to an Amazon SES-operated domain. See below. **NOTE:** This functionality is best effort.
        pub tracking_options: pulumi_wasm_rust::Output<
            Option<super::super::types::ses::ConfigurationSetTrackingOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationSetArgs) -> ConfigurationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delivery_options_binding = args.delivery_options.get_inner();
        let name_binding = args.name.get_inner();
        let reputation_metrics_enabled_binding = args
            .reputation_metrics_enabled
            .get_inner();
        let sending_enabled_binding = args.sending_enabled.get_inner();
        let tracking_options_binding = args.tracking_options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/configurationSet:ConfigurationSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deliveryOptions".into(),
                    value: &delivery_options_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "reputationMetricsEnabled".into(),
                    value: &reputation_metrics_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sendingEnabled".into(),
                    value: &sending_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "trackingOptions".into(),
                    value: &tracking_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "deliveryOptions".into(),
                },
                register_interface::ResultField {
                    name: "lastFreshStart".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "reputationMetricsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sendingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "trackingOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            delivery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryOptions").unwrap(),
            ),
            last_fresh_start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastFreshStart").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            reputation_metrics_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reputationMetricsEnabled").unwrap(),
            ),
            sending_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendingEnabled").unwrap(),
            ),
            tracking_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackingOptions").unwrap(),
            ),
        }
    }
}
