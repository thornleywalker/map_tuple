//! # Map Tuple
//!
//! This crate provides traits that allow `map()`ing of tuple elements of different types to other types like so:
//! ```rust
//! use map_tuple::*;
//!
//! let tuple = (0i32, 1.0f32, 2i32, true, 4i32)
//!     .map3(|val| if val {3i64} else {0})
//!     .map0(|val| val.to_string())
//!     .map1(|val| Some(val))
//!     .map4(|val| val > 3);
//!
//! assert_eq!(tuple, ("0".to_string(), Some(1.0f32), 2i32, 3i64, true));
//! ```
//!
//! # Features
//! Because rust doesn't allow reasoning about tuples generically, each tuple trait has to be
//! implemented for each size of tuple explicitly. This crate provides 4 levels of tuple sizing
//! (which includes all sizes of tuples below it):
//! - 8 (default, no features enabled)
//! - 16 (feature tuple16)
//! - 32 (feature tuple32)
//! - 64 (feature tuple64)
//! - 128 (feature tuple128)
//!
//! Adding additional sizes of tuples is trivially easy, so arbitrary sizes were chosen. In my
//! experience, tuples don't get much larger than 5-10, so having up to 128 available should be
//! beyond sufficient for most, if not all, purposes.
//!
//! To save my sanity, macros are used to simplify this process and make it more scalable. However,
//! this does add some additional compilation time as the macros are expanded. To compound this,
//! compiling rust generics tends to take a long time in and of itself, so every addition of
//! another size of tuple increases compilation time exponentially.
//!
//! For example, on an M1 Mac, the following are the debug compilation times for the various
//! features/tuple sizes:
//! - no features (8) => 0.09s
//! - tuple16 => 0.23s
//! - tuple32 => 1.27s
//! - tuple64 => 9.37s
//! - tuple128 => 84s
//!
//! If, for some unholy reason, you happen to need tuples larger than 128, I highly recommend you
//! reconsider and try to use a struct, vec, enum, or some combination of those. However, if you
//! really want it, a pull request can be created, and larger tuples can be added in a new release
//! (this will not be considered a breaking change).

// We really abuse the macros around here
#![recursion_limit = "512"]

use paste::paste;

macro_rules! tuple_trait {
    ($trait_number:literal) => {
        paste! {
            pub trait [<TupleMap $trait_number >]<R, F> {
                type Output;

                fn [<map $trait_number >](self, f: F) -> Self::Output;
            }
        }
    };
    ($($trait_number:literal),*) => {
        $(tuple_trait!($trait_number);)*
    }
}

macro_rules! impl_trait {
    ($($all:literal),*) => {
        impl_trait!(s () $($all),*);
    };
    // shifting macro
    (s ($($starter:literal),*) $curr:literal, $next:literal $(,$finisher:literal)*) => {
        paste! {
            /// Generics may not line up exactly with the index due to the way the macros are designed, but the size of the tuple is correct
            impl<R, F, $([<T $starter>],)* [<T $curr>], [<T $next>], $([<T $finisher>]),*>
                [<TupleMap $curr>]<R, F> for ($([<T $starter>],)* [<T $curr>], [<T $next>], $([<T $finisher>]),*)
            where
                F: Fn([<T $curr>]) -> R,
            {
                type Output = ($([<T $starter>],)* R, [<T $next>], $([<T $finisher>]),*);
                fn [<map $curr>](self, f: F) -> Self::Output {
                    let ($([<self $starter>],)* [<self $curr>], [<self $next>], $([<self $finisher>]),* )= self;

                    ($([<self $starter>],)* f([<self $curr>]), [<self $next>], $([<self $finisher>]),* )
                }
            }

            impl_trait!(c ($($starter),*) $curr, $($finisher),*);

            impl_trait!(s ($($starter,)* $curr) $next, $($finisher),*);
        }
    };
    // shifting base case
    (s ($($starter:literal),*) $curr:literal,) => {
        paste! {
            impl<R, F, $([<T $starter>],)* [<T $curr>]>
                [<TupleMap $curr>]<R, F> for ($([<T $starter>],)* [<T $curr>],)
            where
                F: Fn([<T $curr>]) -> R,
            {
                type Output = ($([<T $starter>],)* R, );
                fn [<map $curr>](self, f: F) -> Self::Output {
                    let ($([<self $starter>],)* [<self $curr>], )= self;

                    ($([<self $starter>],)* f([<self $curr>]),  )
                }
            }
        }
    };

    // cutting macro
    (c ($($starter:literal),*) $curr:literal, $next:literal $(,$finisher:literal)*) => {
        paste! {
            impl<R, F, $([<T $starter>],)* [<T $curr>], [<T $next>], $([<T $finisher>]),*>
                [<TupleMap $curr>]<R, F> for ($([<T $starter>],)* [<T $curr>], [<T $next>], $([<T $finisher>]),*)
            where
                F: Fn([<T $curr>]) -> R,
            {
                type Output = ($([<T $starter>],)* R, [<T $next>], $([<T $finisher>]),*);
                fn [<map $curr>](self, f: F) -> Self::Output {
                    let ($([<self $starter>],)* [<self $curr>], [<self $next>], $([<self $finisher>]),* )= self;

                    ($([<self $starter>],)* f([<self $curr>]), [<self $next>], $([<self $finisher>]),* )
                }
            }

            impl_trait!(c ($($starter),*) $curr, $($finisher),*);
        }
    };
    // cutting base case
    (c ($($starter:literal),*) $curr:literal,) => {
        paste! {
            impl<R, F, $([<T $starter>],)* [<T $curr>]>
                [<TupleMap $curr>]<R, F> for ($([<T $starter>],)* [<T $curr>],)
            where
                F: Fn([<T $curr>]) -> R,
            {
                type Output = ($([<T $starter>],)* R, );
                fn [<map $curr>](self, f: F) -> Self::Output {
                    let ($([<self $starter>],)* [<self $curr>], )= self;

                    ($([<self $starter>],)* f([<self $curr>]),  )
                }
            }
        }
    };
}

macro_rules! do_all_for_trait {
    ($($all:literal),*) => {
        tuple_trait!($($all),*);
        impl_trait!($($all),*);
    };
}

#[cfg(not(feature = "tuple16"))]
do_all_for_trait!(0, 1, 2, 3, 4, 5, 6, 7);
#[cfg(all(feature = "tuple16", not(feature = "tuple32"),))]
do_all_for_trait!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
#[cfg(all(feature = "tuple32", not(feature = "tuple64"),))]
do_all_for_trait!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31
);
#[cfg(all(feature = "tuple64", not(feature = "tuple128"),))]
do_all_for_trait!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
);
#[cfg(feature = "tuple128")]
do_all_for_trait!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127
);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tuples_to_8() {
        let _tuple = (0, 1, 2, 3, 4, 5, 6, 7)
            .map7(|val| val.to_string())
            .map3(|val| val as u32)
            .map2(|val| val as f64 * 3.5);
    }

    #[cfg(feature = "tuple16")]
    #[test]
    fn tuples_to_16() {
        let _tuple = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
            .map7(|val| val.to_string())
            .map3(|val| val as u32)
            .map2(|val| val as f64 * 3.5)
            .map8(|val| val.to_string())
            .map15(|val| val as u32)
            .map0(|val| val as f64 * 3.5);
    }

    #[cfg(feature = "tuple32")]
    #[test]
    fn tuples_to_32() {
        let _tuple = (
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        )
            .map7(|val| val.to_string())
            .map3(|val| val as u32)
            .map2(|val| val as f64 * 3.5)
            .map8(|val| val.to_string())
            .map15(|val| val as u32)
            .map0(|val| val as f64 * 3.5)
            .map16(|val| val.to_string())
            .map29(|val| val as u32)
            .map31(|val| val as f64 * 3.5);
    }

    #[cfg(feature = "tuple64")]
    #[test]
    fn tuples_to_64() {
        let _tuple = (
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
        )
            .map7(|val| val.to_string())
            .map3(|val| val as u32)
            .map2(|val| val as f64 * 3.5)
            .map8(|val| val.to_string())
            .map15(|val| val as u32)
            .map0(|val| val as f64 * 3.5)
            .map16(|val| val.to_string())
            .map29(|val| val as u32)
            .map31(|val| val as f64 * 3.5)
            .map32(|val| val.to_string())
            .map45(|val| val as u32)
            .map63(|val| val as f64 * 3.5);
    }

    #[cfg(feature = "tuple128")]
    #[test]
    fn tuples_to_128() {
        let _tuple = (
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127,
        )
            .map7(|val| val.to_string())
            .map3(|val| val as u32)
            .map2(|val| val as f64 * 3.5)
            .map8(|val| val.to_string())
            .map15(|val| val as u32)
            .map0(|val| val as f64 * 3.5)
            .map16(|val| val.to_string())
            .map29(|val| val as u32)
            .map31(|val| val as f64 * 3.5)
            .map32(|val| val.to_string())
            .map45(|val| val as u32)
            .map63(|val| val as f64 * 3.5)
            .map89(|val| val.to_string())
            .map110(|val| val as u32)
            .map127(|val| val as f64 * 3.5);
    }
}
