/// Provides a resource to manage an [AWS Macie Account](https://docs.aws.amazon.com/macie/latest/APIReference/macie.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = account::create(
///         "test",
///         AccountArgs::builder()
///             .finding_publishing_frequency("FIFTEEN_MINUTES")
///             .status("ENABLED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_account` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:macie2/account:Account example abcd1
/// ```
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// Specifies how often to publish updates to policy findings for the account. This includes publishing updates to AWS Security Hub and Amazon EventBridge (formerly called Amazon CloudWatch Events). Valid values are `FIFTEEN_MINUTES`, `ONE_HOUR` or `SIX_HOURS`.
        #[builder(into, default)]
        pub finding_publishing_frequency: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The date and time, in UTC and extended RFC 3339 format, when the Amazon Macie account was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Specifies how often to publish updates to policy findings for the account. This includes publishing updates to AWS Security Hub and Amazon EventBridge (formerly called Amazon CloudWatch Events). Valid values are `FIFTEEN_MINUTES`, `ONE_HOUR` or `SIX_HOURS`.
        pub finding_publishing_frequency: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the service-linked role that allows Macie to monitor and analyze data in AWS resources for the account.
        pub service_role: pulumi_wasm_rust::Output<String>,
        /// Specifies the status for the account. To enable Amazon Macie and start all Macie activities for the account, set this value to `ENABLED`. Valid values are `ENABLED` or `PAUSED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The date and time, in UTC and extended RFC 3339 format, of the most recent change to the status of the Macie account.
        pub updated_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccountArgs) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let finding_publishing_frequency_binding = args
            .finding_publishing_frequency
            .get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:macie2/account:Account".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "findingPublishingFrequency".into(),
                    value: &finding_publishing_frequency_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "findingPublishingFrequency".into(),
                },
                register_interface::ResultField {
                    name: "serviceRole".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "updatedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            finding_publishing_frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("findingPublishingFrequency").unwrap(),
            ),
            service_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRole").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedAt").unwrap(),
            ),
        }
    }
}