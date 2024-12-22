//! test new feature with resoruces

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FooArgs {
    #[builder(into, default)]
    pub argument: pulumi_wasm_rust::Output<Option<String>>,
    /// Options for tuning the Kubernetes client used by a Provider.
    #[builder(into)]
    pub backup_kube_client_settings: pulumi_wasm_rust::Output<crate::types::KubeClientSettings>,
    /// Options for tuning the Kubernetes client used by a Provider.
    #[builder(into, default)]
    pub kube_client_settings: pulumi_wasm_rust::Output<Option<crate::types::KubeClientSettings>>,
    /// describing things
    #[builder(into, default)]
    pub settings: pulumi_wasm_rust::Output<Option<crate::types::LayeredType>>,
}

pub struct FooResult {
    /// A test for plain types
    pub default_kube_client_settings: pulumi_wasm_rust::Output<Option<crate::types::KubeClientSettings>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: FooArgs
) -> FooResult {

    let result = crate::bindings::pulumi::example::foo::invoke(
        name,
        &crate::bindings::pulumi::example::foo::Args {
                argument: &args.argument.get_inner(),
                backup_kube_client_settings: &args.backup_kube_client_settings.get_inner(),
                kube_client_settings: &args.kube_client_settings.get_inner(),
                settings: &args.settings.get_inner(),
        }
    );

    FooResult {
        default_kube_client_settings: crate::into_domain(result.default_kube_client_settings),
    }
}
