#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData {
    owner     : Uuid,
    bucket_id : Uuid,
    name      : String,
    vnode     : u64
}

// Define a newtype for custom deserialization
#[derive(Debug, Serialize)]
struct Numberish(pub u64);

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectData2 {
    owner     : Uuid,
    bucket_id : Uuid,
    name      : String,
    vnode     : Numberish
}

struct NumberishVisitor;

// Define custom deserialization for the Numberish type to be more liberal in
// the input accepted. Specifically handle numberic data that may be represented
// as a number or as a string.
impl<'de> Visitor<'de> for NumberishVisitor {
    type Value = Numberish;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 2^127")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(value as u64))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(u64::from(value)))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(value as u64))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(u64::from(value)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(value as u64))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(u64::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Numberish(value as u64))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.parse::<u64>() {
            Ok(parsed_value) => Ok(Numberish(parsed_value)),
            Err(_) => Err(E::custom(format!("string cannot be converted to Numberish: {}", value)))
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.parse::<u64>() {
            Ok(parsed_value) => Ok(Numberish(parsed_value)),
            Err(_) => Err(E::custom(format!("string cannot be converted to Numberish: {}", value)))
        }
    }
}

impl<'de> Deserialize<'de> for Numberish {
    fn deserialize<D>(deserializer: D) -> Result<Numberish, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberishVisitor)
    }
}


fn main() {
    let input1 = "{\"owner\":\"14aafd84-a57f-11e8-8706-4fc23c74c5e7\", \
                  \"bucket_id\":\"ce0dc791-d83f-4395-8e85-cb9edddee542\",\
                  \"name\":\"someobject\",\"vnode\":84}";
    let input2 = "{\"owner\":\"14aafd84-a57f-11e8-8706-4fc23c74c5e7\", \
                  \"bucket_id\":\"ce0dc791-d83f-4395-8e85-cb9edddee542\",\
                  \"name\":\"someobject\",\"vnode\":\"84\"}";

    let result1: Result<ObjectData, _> = serde_json::from_str(input1);
    let result2: Result<ObjectData, _> = serde_json::from_str(input2);
    let result3: Result<ObjectData2, _> = serde_json::from_str(input1);
    let result4: Result<ObjectData2, _> = serde_json::from_str(input2);

    match result1 {
        Ok(object_data1) => println!("Deserialize successful for input1. Vnode: {}", object_data1.vnode),
        Err(err) => println!("Error during input1 deserialization: {}", err)
    };

    match result2 {
        Ok(object_data2) => println!("Deserialize successful for input2. Vnode: {}", object_data2.vnode),
        Err(err) => println!("Error during input2 deserialization: {}", err)
    };

    match result3 {
        Ok(object_data3) => println!("Custom deserialize successful for input1. Vnode: {}", object_data3.vnode.0),
        Err(err) => println!("Error during custom input1 deserialization: {}", err)
    };

    match result4 {
        Ok(object_data4) => println!("Custom deserialize successful for input2. Vnode: {}", object_data4.vnode.0),
        Err(err) => println!("Error during custom input2 deserialization: {}", err)
    };
}
