use std::convert::*;
use super::pixel::*;
use super::rgb::RGB;
use super::rgba::RGBA;

macro_rules! rgb_impl_from {
    ($typename:ident, $from:ty, $to:ty) => {
        impl From<$typename<$from>> for $typename<$to> {
            fn from(other: $typename<$from>) -> Self {
                other.map(|c|c.into())
            }
        }
    }
}

rgb_impl_from!{RGB, u8,u16}
rgb_impl_from!{RGB, u8,i16}
rgb_impl_from!{RGB, u16,i32}
rgb_impl_from!{RGB, u16,u32}

rgb_impl_from!{RGB, u8,f32}
rgb_impl_from!{RGB, u8,f64}
rgb_impl_from!{RGB, u16,f32}
rgb_impl_from!{RGB, u16,f64}

rgb_impl_from!{RGB, i8,f32}
rgb_impl_from!{RGB, i8,f64}
rgb_impl_from!{RGB, i16,f32}
rgb_impl_from!{RGB, i16,f64}

rgb_impl_from!{RGB, i32,f64}
rgb_impl_from!{RGB, f32,f64}


rgb_impl_from!{RGBA, u8,u16}
rgb_impl_from!{RGBA, u8,i16}
rgb_impl_from!{RGBA, u16,i32}
rgb_impl_from!{RGBA, u16,u32}

rgb_impl_from!{RGBA, u8,f32}
rgb_impl_from!{RGBA, u8,f64}
rgb_impl_from!{RGBA, u16,f32}
rgb_impl_from!{RGBA, u16,f64}

rgb_impl_from!{RGBA, i8,f32}
rgb_impl_from!{RGBA, i8,f64}
rgb_impl_from!{RGBA, i16,f32}
rgb_impl_from!{RGBA, i16,f64}

rgb_impl_from!{RGBA, i32,f64}
rgb_impl_from!{RGBA, f32,f64}