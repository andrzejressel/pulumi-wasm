/// Manages a Kinesis Analytics v2 Application Snapshot.
/// Snapshots are the AWS implementation of [Flink Savepoints](https://ci.apache.org/projects/flink/flink-docs-release-1.11/ops/state/savepoints.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationSnapshotArgs {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        #[builder(into)]
        pub application_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the application snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationSnapshotResult {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        pub application_name: pulumi_gestalt_rust::Output<String>,
        /// The current application version ID when the snapshot was created.
        pub application_version_id: pulumi_gestalt_rust::Output<i32>,
        /// The timestamp of the application snapshot.
        pub snapshot_creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The name of the application snapshot.
        pub snapshot_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationSnapshotArgs,
    ) -> ApplicationSnapshotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_name_binding = args
            .application_name
            .get_output(context)
            .get_inner();
        let snapshot_name_binding = args.snapshot_name.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationSnapshotResult {
            application_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationName"),
            ),
            application_version_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationVersionId"),
            ),
            snapshot_creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotCreationTimestamp"),
            ),
            snapshot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotName"),
            ),
        }
    }
}
