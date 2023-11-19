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
//! For example, on an i5-8400, the following are the debug compilation times for the various
//! features/tuple sizes:
//! - no features (8) => 0.09s
//! - tuple16 => 0.24s
//! - tuple32 => 1.26s
//! - tuple64 => 9.21s
//! - tuple128 => 76s
//!
//! If, for some unholy reason, you happen to need tuples larger than 128, I highly recommend you
//! reconsider and try to use a struct, vec, enum, or some combination of those. However, if you
//! really want it, a pull request can be created, and larger tuples can be added in a new release
//! (this will not be considered a breaking change).

// We really abuse the macros around here
#![recursion_limit = "512"]

use paste::paste;

/// Copy-pasted from https://stackoverflow.com/a/42176533/13622927.
/// This is a necessary hack to be able to iterate over macro arguments from last to first.
macro_rules! apply_args_reverse {
    ($macro_id:tt [] $($reversed:tt)*) => {
        $macro_id!($($reversed) *);
    };
    ($macro_id:tt [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
        apply_args_reverse!($macro_id [$($rest)*] $first $($reversed)*);
    };
    // Entry point, use brackets to recursively reverse above.
    ($macro_id:tt, $($t:tt)*) => {
        apply_args_reverse!($macro_id [ $($t)* ]);
    };
}

/// Accept one or more numbers and generate the corresponding TupleMap trait(s).
macro_rules! tuple_trait {
    ($i:literal) => {
        paste! {
            pub trait [<TupleMap$i>]<F> {
                type Output;
                fn [<map$i>](self, f: F) -> Self::Output;
            }
        }
    };
    ($($trait_number:literal),*) => {
        $(tuple_trait!($trait_number);)*
    }
}

/// Accept numbers (N N-1 ... 1 0), treat them as tuple sizes,
/// and for each tuple size implement all possible MapTuple traits.
macro_rules! impl_traits_for_tuples {
    // Recurisively iterate tuple sizes from first (the largest) to last (the smallest).
    ($max_size:literal $($smaller_sizes:literal)*) => {
        apply_args_reverse!(impl_traits_for_tuple, $max_size $($smaller_sizes)*);
        impl_traits_for_tuples!($($smaller_sizes)*);
    };
    // Base case.
    () => {}
}

/// Accept numbers (0 1 ... N-1 N) and implement all possible MapTuple traits for tuple of size N.
macro_rules! impl_traits_for_tuple {
    // "Public" case as advertized above.
    ($zero:literal $($positive_nums:literal)*) => {
        impl_traits_for_tuple!( | $zero $($positive_nums)*);
    };
    // "Private" case - recursively move forward through the arguments.
    // The '|' determines the current position in the argument list.
    ($($before:literal)* | $i:literal $($after:literal)*) => {
        paste! {
            impl<R, F, $([<T$before>],)* [<T$i>], $([<T$after>],)*>
                [<TupleMap$i>]<F> for ($([<T$before>],)* [<T$i>], $([<T$after>],)*)
            where
                F: FnOnce([<T$i>]) -> R,
            {
                type Output = ($([<T$before>],)* R, $([<T$after>],)*);
                fn [<map$i>](self, f: F) -> Self::Output {
                    ($(self.$before,)* f(self.$i), $(self.$after,)*)
                }
            }
        }
        impl_traits_for_tuple!($($before)* $i | $($after)*);
    };
    // "Private" base case: we've iterated over the entire argument list.
    ($($implemented:literal)+ | ) => {};
}

/// Accept numbers (0, 1, ..., N-1, N), define all corresponding MapTuple traits
/// and implement them for all corresponding tuple sizes.
macro_rules! do_all_for_trait {
    ($($all:literal),*) => {
        tuple_trait!($($all),*);
        apply_args_reverse!(impl_traits_for_tuples, $($all)*);
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

    #[test]
    fn should_work_with_fn_once_closure() {
        let iter = [1, 2, 3].into_iter();
        // Closure captures `iter` by value and can only be called once.
        assert_eq!(("foo", 4).map1(|n| iter.count() == n), ("foo", false));
    }

    #[test]
    fn should_work_with_fn_mut_closure() {
        let mut iter = [1, 2, 3].into_iter();
        // Closure captures `iter` by mutable reference.
        assert_eq!(("foo", 1).map1(|n| iter.next() == Some(n)), ("foo", true));
        assert_eq!(iter.collect::<Vec<i32>>(), vec![2, 3]);
    }

    #[test]
    fn should_work_with_fn_closure() {
        let allowed = ["foo", "bar", "baz"];
        // Closure captures `allowed` by immutable reference.
        assert_eq!(("foo", 1).map0(|s| allowed.contains(&s)), (true, 1));
    }

    #[test]
    fn should_work_with_fn_pointer() {
        assert_eq!(("foo", 1).map1(i32::is_positive), ("foo", true));
    }
}
