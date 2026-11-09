// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Lance Authors

//! Minimal scalar values for v1 statistics.
//!
//! This replaces `datafusion_common::ScalarValue` with the small subset the
//! legacy statistics collector needs. Keeping it local lets `lance-file` build
//! without any DataFusion crate.

use std::cmp::Ordering;
use std::sync::Arc;

use arrow_array::ArrowPrimitiveType;
use arrow_array::types::{
    Date32Type, Date64Type, Decimal128Type, DurationMicrosecondType, DurationMillisecondType,
    DurationNanosecondType, DurationSecondType, Float32Type, Float64Type, Int8Type, Int16Type,
    Int32Type, Int64Type, Time32MillisecondType, Time32SecondType, Time64MicrosecondType,
    Time64NanosecondType, TimestampMicrosecondType, TimestampMillisecondType,
    TimestampNanosecondType, TimestampSecondType, UInt8Type, UInt16Type, UInt32Type, UInt64Type,
};
#[cfg(test)]
use arrow_array::{
    ArrayRef, BinaryArray, BooleanArray, Date32Array, Date64Array, Decimal128Array,
    DurationMicrosecondArray, DurationMillisecondArray, DurationNanosecondArray,
    DurationSecondArray, FixedSizeBinaryArray, Float32Array, Float64Array, Int8Array, Int16Array,
    Int32Array, Int64Array, LargeBinaryArray, LargeStringArray, Scalar, StringArray,
    Time32MillisecondArray, Time32SecondArray, Time64MicrosecondArray, Time64NanosecondArray,
    TimestampMicrosecondArray, TimestampMillisecondArray, TimestampNanosecondArray,
    TimestampSecondArray, UInt8Array, UInt16Array, UInt32Array, UInt64Array,
};
use arrow_schema::DataType;

#[derive(Debug, Clone)]
pub enum ScalarValue {
    Boolean(Option<bool>),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    UInt8(Option<u8>),
    UInt16(Option<u16>),
    UInt32(Option<u32>),
    UInt64(Option<u64>),
    Float32(Option<f32>),
    Float64(Option<f64>),
    Date32(Option<i32>),
    Date64(Option<i64>),
    Time32Second(Option<i32>),
    Time32Millisecond(Option<i32>),
    Time64Microsecond(Option<i64>),
    Time64Nanosecond(Option<i64>),
    #[allow(dead_code)]
    TimestampSecond(Option<i64>, Option<Arc<str>>),
    #[allow(dead_code)]
    TimestampMillisecond(Option<i64>, Option<Arc<str>>),
    #[allow(dead_code)]
    TimestampMicrosecond(Option<i64>, Option<Arc<str>>),
    #[allow(dead_code)]
    TimestampNanosecond(Option<i64>, Option<Arc<str>>),
    DurationSecond(Option<i64>),
    DurationMillisecond(Option<i64>),
    DurationMicrosecond(Option<i64>),
    DurationNanosecond(Option<i64>),
    Utf8(Option<String>),
    LargeUtf8(Option<String>),
    Binary(Option<Vec<u8>>),
    LargeBinary(Option<Vec<u8>>),
    #[allow(dead_code)]
    FixedSizeBinary(i32, Option<Vec<u8>>),
    Decimal128(Option<i128>, u8, i8),
}

macro_rules! scalar_eq {
    ($left:expr, $right:expr, $($variant:ident),* $(,)?) => {
        match ($left, $right) {
            $( (ScalarValue::$variant(a), ScalarValue::$variant(b)) => a == b, )*
            _ => false,
        }
    };
}

impl PartialEq for ScalarValue {
    fn eq(&self, other: &Self) -> bool {
        use ScalarValue::*;
        match (self, other) {
            (Float32(a), Float32(b)) => match (a, b) {
                (Some(a), Some(b)) => a.to_bits() == b.to_bits(),
                _ => a == b,
            },
            (Float64(a), Float64(b)) => match (a, b) {
                (Some(a), Some(b)) => a.to_bits() == b.to_bits(),
                _ => a == b,
            },
            (FixedSizeBinary(_, a), FixedSizeBinary(_, b)) => a == b,
            (Decimal128(a, p1, s1), Decimal128(b, p2, s2)) => a == b && p1 == p2 && s1 == s2,
            (TimestampSecond(a, _), TimestampSecond(b, _)) => a == b,
            (TimestampMillisecond(a, _), TimestampMillisecond(b, _)) => a == b,
            (TimestampMicrosecond(a, _), TimestampMicrosecond(b, _)) => a == b,
            (TimestampNanosecond(a, _), TimestampNanosecond(b, _)) => a == b,
            _ => scalar_eq!(
                self,
                other,
                Boolean,
                Int8,
                Int16,
                Int32,
                Int64,
                UInt8,
                UInt16,
                UInt32,
                UInt64,
                Date32,
                Date64,
                Time32Second,
                Time32Millisecond,
                Time64Microsecond,
                Time64Nanosecond,
                DurationSecond,
                DurationMillisecond,
                DurationMicrosecond,
                DurationNanosecond,
                Utf8,
                LargeUtf8,
                Binary,
                LargeBinary,
            ),
        }
    }
}

macro_rules! scalar_ord {
    ($left:expr, $right:expr, $($variant:ident),* $(,)?) => {
        match ($left, $right) {
            $( (ScalarValue::$variant(a), ScalarValue::$variant(b)) => a.partial_cmp(b), )*
            _ => None,
        }
    };
}

impl PartialOrd for ScalarValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use ScalarValue::*;
        match (self, other) {
            (Float32(a), Float32(b)) => match (a, b) {
                (Some(a), Some(b)) => Some(a.total_cmp(b)),
                _ => a.partial_cmp(b),
            },
            (Float64(a), Float64(b)) => match (a, b) {
                (Some(a), Some(b)) => Some(a.total_cmp(b)),
                _ => a.partial_cmp(b),
            },
            (FixedSizeBinary(_, a), FixedSizeBinary(_, b)) => a.partial_cmp(b),
            (Decimal128(a, _, s1), Decimal128(b, _, s2)) => {
                if s1 == s2 {
                    a.partial_cmp(b)
                } else {
                    None
                }
            }
            (TimestampSecond(a, _), TimestampSecond(b, _)) => a.partial_cmp(b),
            (TimestampMillisecond(a, _), TimestampMillisecond(b, _)) => a.partial_cmp(b),
            (TimestampMicrosecond(a, _), TimestampMicrosecond(b, _)) => a.partial_cmp(b),
            (TimestampNanosecond(a, _), TimestampNanosecond(b, _)) => a.partial_cmp(b),
            _ => scalar_ord!(
                self,
                other,
                Boolean,
                Int8,
                Int16,
                Int32,
                Int64,
                UInt8,
                UInt16,
                UInt32,
                UInt64,
                Date32,
                Date64,
                Time32Second,
                Time32Millisecond,
                Time64Microsecond,
                Time64Nanosecond,
                DurationSecond,
                DurationMillisecond,
                DurationMicrosecond,
                DurationNanosecond,
                Utf8,
                LargeUtf8,
                Binary,
                LargeBinary,
            ),
        }
    }
}

#[cfg(test)]
impl ScalarValue {
    pub(crate) fn try_new_decimal128(value: i128, precision: u8, scale: i8) -> Option<Self> {
        (precision > 0 && precision <= 38).then_some(Self::Decimal128(
            Some(value),
            precision,
            scale,
        ))
    }

    pub(crate) fn is_null(&self) -> bool {
        match self {
            Self::Boolean(v) => v.is_none(),
            Self::Int8(v) => v.is_none(),
            Self::Int16(v) => v.is_none(),
            Self::Int32(v) => v.is_none(),
            Self::Int64(v) => v.is_none(),
            Self::UInt8(v) => v.is_none(),
            Self::UInt16(v) => v.is_none(),
            Self::UInt32(v) => v.is_none(),
            Self::UInt64(v) => v.is_none(),
            Self::Float32(v) => v.is_none(),
            Self::Float64(v) => v.is_none(),
            Self::Date32(v) => v.is_none(),
            Self::Date64(v) => v.is_none(),
            Self::Time32Second(v) => v.is_none(),
            Self::Time32Millisecond(v) => v.is_none(),
            Self::Time64Microsecond(v) => v.is_none(),
            Self::Time64Nanosecond(v) => v.is_none(),
            Self::TimestampSecond(v, _) => v.is_none(),
            Self::TimestampMillisecond(v, _) => v.is_none(),
            Self::TimestampMicrosecond(v, _) => v.is_none(),
            Self::TimestampNanosecond(v, _) => v.is_none(),
            Self::DurationSecond(v) => v.is_none(),
            Self::DurationMillisecond(v) => v.is_none(),
            Self::DurationMicrosecond(v) => v.is_none(),
            Self::DurationNanosecond(v) => v.is_none(),
            Self::Utf8(v) => v.is_none(),
            Self::LargeUtf8(v) => v.is_none(),
            Self::Binary(v) => v.is_none(),
            Self::LargeBinary(v) => v.is_none(),
            Self::FixedSizeBinary(_, v) => v.is_none(),
            Self::Decimal128(v, _, _) => v.is_none(),
        }
    }
}

#[cfg(test)]
impl ScalarValue {
    pub(crate) fn to_scalar(&self) -> Scalar<ArrayRef> {
        Scalar::new(self.to_array())
    }

    fn to_array(&self) -> ArrayRef {
        match self {
            Self::Boolean(value) => Arc::new(BooleanArray::from(vec![*value])) as ArrayRef,
            Self::Int8(value) => Arc::new(Int8Array::from(vec![*value])) as ArrayRef,
            Self::Int16(value) => Arc::new(Int16Array::from(vec![*value])) as ArrayRef,
            Self::Int32(value) => Arc::new(Int32Array::from(vec![*value])) as ArrayRef,
            Self::Int64(value) => Arc::new(Int64Array::from(vec![*value])) as ArrayRef,
            Self::UInt8(value) => Arc::new(UInt8Array::from(vec![*value])) as ArrayRef,
            Self::UInt16(value) => Arc::new(UInt16Array::from(vec![*value])) as ArrayRef,
            Self::UInt32(value) => Arc::new(UInt32Array::from(vec![*value])) as ArrayRef,
            Self::UInt64(value) => Arc::new(UInt64Array::from(vec![*value])) as ArrayRef,
            Self::Float32(value) => Arc::new(Float32Array::from(vec![*value])) as ArrayRef,
            Self::Float64(value) => Arc::new(Float64Array::from(vec![*value])) as ArrayRef,
            Self::Date32(value) => Arc::new(Date32Array::from(vec![*value])) as ArrayRef,
            Self::Date64(value) => Arc::new(Date64Array::from(vec![*value])) as ArrayRef,
            Self::Time32Second(value) => {
                Arc::new(Time32SecondArray::from(vec![*value])) as ArrayRef
            }
            Self::Time32Millisecond(value) => {
                Arc::new(Time32MillisecondArray::from(vec![*value])) as ArrayRef
            }
            Self::Time64Microsecond(value) => {
                Arc::new(Time64MicrosecondArray::from(vec![*value])) as ArrayRef
            }
            Self::Time64Nanosecond(value) => {
                Arc::new(Time64NanosecondArray::from(vec![*value])) as ArrayRef
            }
            Self::TimestampSecond(value, timezone) => Arc::new(
                TimestampSecondArray::from(vec![*value]).with_timezone_opt(timezone.clone()),
            ) as ArrayRef,
            Self::TimestampMillisecond(value, timezone) => Arc::new(
                TimestampMillisecondArray::from(vec![*value]).with_timezone_opt(timezone.clone()),
            ) as ArrayRef,
            Self::TimestampMicrosecond(value, timezone) => Arc::new(
                TimestampMicrosecondArray::from(vec![*value]).with_timezone_opt(timezone.clone()),
            ) as ArrayRef,
            Self::TimestampNanosecond(value, timezone) => Arc::new(
                TimestampNanosecondArray::from(vec![*value]).with_timezone_opt(timezone.clone()),
            ) as ArrayRef,
            Self::DurationSecond(value) => {
                Arc::new(DurationSecondArray::from(vec![*value])) as ArrayRef
            }
            Self::DurationMillisecond(value) => {
                Arc::new(DurationMillisecondArray::from(vec![*value])) as ArrayRef
            }
            Self::DurationMicrosecond(value) => {
                Arc::new(DurationMicrosecondArray::from(vec![*value])) as ArrayRef
            }
            Self::DurationNanosecond(value) => {
                Arc::new(DurationNanosecondArray::from(vec![*value])) as ArrayRef
            }
            Self::Utf8(value) => Arc::new(StringArray::from(vec![value.clone()])) as ArrayRef,
            Self::LargeUtf8(value) => {
                Arc::new(LargeStringArray::from(vec![value.clone()])) as ArrayRef
            }
            Self::Binary(value) => {
                Arc::new(BinaryArray::from_opt_vec(vec![value.as_deref()])) as ArrayRef
            }
            Self::LargeBinary(value) => {
                Arc::new(LargeBinaryArray::from_opt_vec(vec![value.as_deref()])) as ArrayRef
            }
            Self::FixedSizeBinary(length, value) => {
                if value.is_none() {
                    arrow_array::new_null_array(&DataType::FixedSizeBinary(*length), 1)
                } else {
                    Arc::new(
                        FixedSizeBinaryArray::try_from_sparse_iter_with_size(
                            vec![value.clone()].into_iter(),
                            *length,
                        )
                        .expect("fixed-size binary statistics value has the wrong length"),
                    ) as ArrayRef
                }
            }
            Self::Decimal128(value, precision, scale) => Arc::new(
                Decimal128Array::from(vec![*value])
                    .with_precision_and_scale(*precision, *scale)
                    .expect("decimal statistics value has invalid precision or scale"),
            ) as ArrayRef,
        }
    }
}

macro_rules! impl_from {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for ScalarValue {
            fn from(value: $ty) -> Self {
                Self::$variant(Some(value))
            }
        }
    };
}

impl_from!(bool, Boolean);
impl_from!(i8, Int8);
impl_from!(i16, Int16);
impl_from!(i32, Int32);
impl_from!(i64, Int64);
impl_from!(u8, UInt8);
impl_from!(u16, UInt16);
impl_from!(u32, UInt32);
impl_from!(u64, UInt64);
impl_from!(f32, Float32);
impl_from!(f64, Float64);

impl From<&str> for ScalarValue {
    fn from(value: &str) -> Self {
        Self::Utf8(Some(value.to_string()))
    }
}

impl From<String> for ScalarValue {
    fn from(value: String) -> Self {
        Self::Utf8(Some(value))
    }
}

impl From<Vec<u8>> for ScalarValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Binary(Some(value))
    }
}

impl From<&[u8]> for ScalarValue {
    fn from(value: &[u8]) -> Self {
        Self::Binary(Some(value.to_vec()))
    }
}

/// Converts between Arrow primitive values and the matching scalar variant.
pub trait StatPrimitive: ArrowPrimitiveType {
    fn to_scalar(value: Self::Native, data_type: &DataType) -> ScalarValue;
    fn from_scalar(value: &ScalarValue) -> Option<Self::Native>;
}

macro_rules! impl_stat_primitive {
    ($ty:ty, $variant:ident) => {
        impl StatPrimitive for $ty {
            fn to_scalar(value: Self::Native, _: &DataType) -> ScalarValue {
                ScalarValue::$variant(Some(value))
            }

            fn from_scalar(value: &ScalarValue) -> Option<Self::Native> {
                match value {
                    ScalarValue::$variant(Some(value)) => Some(*value),
                    _ => None,
                }
            }
        }
    };
}

impl_stat_primitive!(Int8Type, Int8);
impl_stat_primitive!(Int16Type, Int16);
impl_stat_primitive!(Int32Type, Int32);
impl_stat_primitive!(Int64Type, Int64);
impl_stat_primitive!(UInt8Type, UInt8);
impl_stat_primitive!(UInt16Type, UInt16);
impl_stat_primitive!(UInt32Type, UInt32);
impl_stat_primitive!(UInt64Type, UInt64);
impl_stat_primitive!(Float32Type, Float32);
impl_stat_primitive!(Float64Type, Float64);
impl_stat_primitive!(Date32Type, Date32);
impl_stat_primitive!(Date64Type, Date64);
impl_stat_primitive!(Time32SecondType, Time32Second);
impl_stat_primitive!(Time32MillisecondType, Time32Millisecond);
impl_stat_primitive!(Time64MicrosecondType, Time64Microsecond);
impl_stat_primitive!(Time64NanosecondType, Time64Nanosecond);
impl_stat_primitive!(DurationSecondType, DurationSecond);
impl_stat_primitive!(DurationMillisecondType, DurationMillisecond);
impl_stat_primitive!(DurationMicrosecondType, DurationMicrosecond);
impl_stat_primitive!(DurationNanosecondType, DurationNanosecond);

macro_rules! impl_timestamp_primitive {
    ($ty:ty, $variant:ident) => {
        impl StatPrimitive for $ty {
            fn to_scalar(value: Self::Native, data_type: &DataType) -> ScalarValue {
                let timezone = match data_type {
                    DataType::Timestamp(_, timezone) => timezone.clone(),
                    _ => None,
                };
                ScalarValue::$variant(Some(value), timezone)
            }

            fn from_scalar(value: &ScalarValue) -> Option<Self::Native> {
                match value {
                    ScalarValue::$variant(Some(value), _) => Some(*value),
                    _ => None,
                }
            }
        }
    };
}

impl_timestamp_primitive!(TimestampSecondType, TimestampSecond);
impl_timestamp_primitive!(TimestampMillisecondType, TimestampMillisecond);
impl_timestamp_primitive!(TimestampMicrosecondType, TimestampMicrosecond);
impl_timestamp_primitive!(TimestampNanosecondType, TimestampNanosecond);

impl StatPrimitive for Decimal128Type {
    fn to_scalar(value: Self::Native, data_type: &DataType) -> ScalarValue {
        let (precision, scale) = match data_type {
            DataType::Decimal128(precision, scale) => (*precision, *scale),
            _ => (38, 0),
        };
        ScalarValue::Decimal128(Some(value), precision, scale)
    }

    fn from_scalar(value: &ScalarValue) -> Option<Self::Native> {
        match value {
            ScalarValue::Decimal128(Some(value), _, _) => Some(*value),
            _ => None,
        }
    }
}
