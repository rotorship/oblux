//! Unsigned integer types with guaranteed signed integer compatibility.

#[rustfmt::skip]
macro_rules! define_ux {
	($name:ident, $unsigned:ty, $signed:ty) => {
		#[doc = concat!(
					"Unsigned integer type with guaranteed [`",
					stringify!($signed),
					"`] compatibility.",
				)]
		#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
		pub struct $name($unsigned);

		impl $name {
			/// The minimum value for this type (always `0`).
			///
			/// ```
			#[doc = concat!("# use oblux::", stringify!($name), ";")]
			#[doc = concat!("assert_eq!(", stringify!($name), "::MIN.get(), ", "0);")]
			/// ```
			pub const MIN: Self = Self(<$unsigned>::MIN);

			/// The maximum value for this type.
			///
			/// ```
			#[doc = concat!("# use oblux::", stringify!($name), ";")]
			#[doc = concat!(
						"assert_eq!(",
						stringify!($name),
						"::MAX.get(), ",
						stringify!($signed),
						"::MAX as ",
						stringify!($unsigned),
						");",
					)]
			/// ```
			pub const MAX: Self = Self(Self::INNER_MAX);

			/// The constant value `1`.
			///
			/// ```
			#[doc = concat!("# use oblux::", stringify!($name), ";")]
			#[doc = concat!("assert_eq!(", stringify!($name), "::ONE.get(), ", "1);")]
			/// ```
			pub const ONE: Self = Self(1);

			/// The constant value `2`.
			///
			/// ```
			#[doc = concat!("# use oblux::", stringify!($name), ";")]
			#[doc = concat!("assert_eq!(", stringify!($name), "::TWO.get(), ", "2);")]
			/// ```
			pub const TWO: Self = Self(2);

			/// The constant value `3`.
			///
			/// ```
			#[doc = concat!("# use oblux::", stringify!($name), ";")]
			#[doc = concat!("assert_eq!(", stringify!($name), "::THREE.get(), ", "3);")]
			/// ```
			pub const THREE: Self = Self(3);

			const INNER_MAX: $unsigned = <$signed>::MAX as $unsigned;

			/// Creates a new instance if `x` is within the valid range.
            ///
            #[doc = concat!(
            			"Returns `None` if magnitude of `x` exceeds [`",
            			stringify!($signed),
            			"::MAX`].",
            		)]
			pub const fn new(x: $unsigned) -> Option<Self> {
				if x <= Self::INNER_MAX {
					Some(Self(x))
				} else {
					None
				}
			}

			/// Creates a new instance if `x` is non-negative.
			pub const fn from_signed(x: $signed) -> Option<Self> {
				if x < 0 {
					None
				} else {
					Self::new(x as $unsigned)
				}
			}

			#[doc = concat!("Returns the underlying [`", stringify!($unsigned), "`].")]
			pub const fn get(self) -> $unsigned {
				self.0
			}

			#[doc = concat!("Returns the corresponding [`", stringify!($signed), "`].")]
            ///
            #[doc = concat!(
	            		"This is always safe because the value is guaranteed to be within the ",
	            		"positive range of [`", stringify!($signed), "`].",
					)]
			pub const fn to_signed(self) -> $signed {
				self.get() as $signed
			}
		}
	};
}

define_ux!(U7, u8, i8);
define_ux!(U15, u16, i16);
define_ux!(U31, u32, i32);
define_ux!(U63, u64, i64);
define_ux!(U127, u128, i128);
