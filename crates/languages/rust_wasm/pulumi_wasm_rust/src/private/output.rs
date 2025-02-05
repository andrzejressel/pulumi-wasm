use crate::Output;
use pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::output_interface;
use serde::Serialize;
use std::ops::Deref;

macro_rules! impl_combine {
    ($($func_name:ident => ($($var_lower:ident : $var_upper:ident),+)),+) => {
        $(
            #[allow(clippy::too_many_arguments)]
            pub fn $func_name<$($var_upper),+>($($var_lower: Output<$var_upper>),+) -> Output<($($var_upper),+)>
            where $($var_upper: Serialize),+ {
                let output_id = output_interface::combine(
                    &[$($var_lower.get_inner().deref()),+],
                );
                unsafe { Output::<($($var_upper),+)>::new_from_handle(output_id) }
            }
        )+
    };
}

impl_combine! {
    combine2 => (a: A, b: B),
    combine3 => (a: A, b: B, c: C),
    combine4 => (a: A, b: B, c: C, d: D),
    combine5 => (a: A, b: B, c: C, d: D, e: E),
    combine6 => (a: A, b: B, c: C, d: D, e: E, f: F),
    combine7 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G),
    combine8 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H),
    combine9 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I),
    combine10 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J),
    combine11 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K),
    combine12 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L),
    combine13 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M),
    combine14 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N),
    combine15 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O),
    combine16 => (a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O, p: P)
}
