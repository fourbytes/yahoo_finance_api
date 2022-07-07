use std::fmt;

use rust_decimal::Decimal;
use serde::de::{Visitor, MapAccess, Deserialize, Deserializer};
use time::{Date, OffsetDateTime};

#[derive(Debug)]
pub struct IntegerValue(pub Option<i64>);

struct IntegerValueVisitor;

impl<'de> Visitor<'de> for IntegerValueVisitor {
    type Value = IntegerValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("object with raw property")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut value = None;
        while let Some(key) = map.next_key::<String>()? {
            if let "raw" = key.as_str() {
                value = Some(map.next_value()?);
            }
        }
        Ok(IntegerValue(value))
    }
}

impl<'de> Deserialize<'de> for IntegerValue {
    fn deserialize<D>(deserializer: D) -> Result<IntegerValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IntegerValueVisitor)
    }
}

#[derive(Debug)]
pub struct DecimalValue(pub Option<Decimal>);

struct DecimalValueVisitor;

impl<'de> Visitor<'de> for DecimalValueVisitor {
    type Value = DecimalValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("object with raw property")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut value = None;
        while let Some(key) = map.next_key::<String>()? {
            if let "raw" = key.as_str() {
                value = Some(map.next_value()?);
            }
        }
        Ok(DecimalValue(value))
    }
}

impl<'de> Deserialize<'de> for DecimalValue {
    fn deserialize<D>(deserializer: D) -> Result<DecimalValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DecimalValueVisitor)
    }
}

#[derive(Debug)]
pub struct OffsetDateTimeValue(pub Option<OffsetDateTime>);

struct DateValueVisitor;

impl<'de> Visitor<'de> for DateValueVisitor {
    type Value = OffsetDateTimeValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("object with raw property")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut value = None;
        while let Some(key) = map.next_key::<String>()? {
            if let "raw" = key.as_str() {
                value = OffsetDateTime::from_unix_timestamp(map.next_value()?).ok();
            }
        }
        Ok(OffsetDateTimeValue(value))
    }
}

impl<'de> Deserialize<'de> for OffsetDateTimeValue {
    fn deserialize<D>(deserializer: D) -> Result<OffsetDateTimeValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DateValueVisitor)
    }
}
