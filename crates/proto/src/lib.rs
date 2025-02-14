#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![cfg(not(doctest))]

use prost::Message;

#[cfg(feature = "connectivity")]
pub mod full;
pub mod mini;

pub trait IntoFull<T> {
    fn into_full(self) -> T;
}

pub trait IntoMini<T> {
    fn into_mini(self) -> T;
}

macro_rules! generate_intos {
    ($src:ident) => {
        #[cfg(feature = "connectivity")]
        impl IntoFull<full::pulumirpc::$src> for mini::pulumirpc::$src {
            fn into_full(self) -> full::pulumirpc::$src {
                let mut buf = self.encode_to_vec();
                self.encode(&mut buf).unwrap();
                full::pulumirpc::$src::decode(&mut buf.as_slice()).unwrap()
            }
        }

        #[cfg(feature = "connectivity")]
        impl IntoMini<mini::pulumirpc::$src> for full::pulumirpc::$src {
            fn into_mini(self) -> mini::pulumirpc::$src {
                let mut buf = self.encode_to_vec();
                self.encode(&mut buf).unwrap();
                mini::pulumirpc::$src::decode(&mut buf.as_slice()).unwrap()
            }
        }
    };
}

generate_intos!(RegisterResourceRequest);
generate_intos!(ResourceInvokeRequest);
generate_intos!(RegisterResourceOutputsRequest);
generate_intos!(GetRootResourceRequest);
generate_intos!(SetRootResourceRequest);
