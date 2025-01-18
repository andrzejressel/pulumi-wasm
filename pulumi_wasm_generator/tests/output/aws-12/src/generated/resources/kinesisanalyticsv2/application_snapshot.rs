/// Manages a Kinesis Analytics v2 Application Snapshot.
/// Snapshots are the AWS implementation of [Flink Savepoints](https://ci.apache.org/projects/flink/flink-docs-release-1.11/ops/state/savepoints.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application_snapshot::create(
///         "example",
///         ApplicationSnapshotArgs::builder()
///             .application_name("${exampleAwsKinesisanalyticsv2Application.name}")
///             .snapshot_name("example-snapshot")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_kinesisanalyticsv2_application` using `application_name` together with `snapshot_name`. For example:
///
/// ```sh
/// $ pulumi import aws:kinesisanalyticsv2/applicationSnapshot:ApplicationSnapshot example example-application/example-snapshot
/// ```
pub mod application_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationSnapshotArgs {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        #[builder(into)]
        pub application_name: pulumi_wasm_rust::Output<String>,
        /// The name of the application snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationSnapshotResult {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        pub application_name: pulumi_wasm_rust::Output<String>,
        /// The current application version ID when the snapshot was created.
        pub application_version_id: pulumi_wasm_rust::Output<i32>,
        /// The timestamp of the application snapshot.
        pub snapshot_creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// The name of the application snapshot.
        pub snapshot_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApplicationSnapshotArgs,
    ) -> ApplicationSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_name_binding = args.application_name.get_inner();
        let snapshot_name_binding = args.snapshot_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kinesisanalyticsv2/applicationSnapshot:ApplicationSnapshot"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationName".into(),
                    value: &application_name_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotName".into(),
                    value: &snapshot_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationName".into(),
                },
                register_interface::ResultField {
                    name: "applicationVersionId".into(),
                },
                register_interface::ResultField {
                    name: "snapshotCreationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "snapshotName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationSnapshotResult {
            application_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationName").unwrap(),
            ),
            application_version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationVersionId").unwrap(),
            ),
            snapshot_creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotCreationTimestamp").unwrap(),
            ),
            snapshot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotName").unwrap(),
            ),
        }
    }
}
