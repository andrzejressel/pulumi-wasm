use crate::Output;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::output_interface;
use serde::Serialize;
use std::ops::Deref;

macro_rules! impl_combine {
    ($($func_name:ident => ($($var_lower:ident : $var_upper:ident),+)),+) => {
        $(
            #[allow(clippy::too_many_arguments)]
            pub fn $func_name<A, $($var_upper),+>(a: Output<A>, $($var_lower: Output<$var_upper>),+) -> Output<($($var_upper),+)>
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
    combine2 => (b: B),
    combine3 => (b: B, c: C),
    combine4 => (b: B, c: C, d: D),
    combine5 => (b: B, c: C, d: D, e: E),
    combine6 => (b: B, c: C, d: D, e: E, f: F),
    combine7 => (b: B, c: C, d: D, e: E, f: F, g: G),
    combine8 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H),
    combine9 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I),
    combine10 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J),
    combine11 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K),
    combine12 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L),
    combine13 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M),
    combine14 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N),
    combine15 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O),
    combine16 => (b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L, m: M, n: N, o: O, p: P)
}
