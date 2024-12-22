

pub struct SomeFunctionResult {
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
) -> SomeFunctionResult {

    let result = crate::bindings::pulumi::typesystem::deep_nested_module_some_function::invoke(
    );

    SomeFunctionResult {
    }
}
