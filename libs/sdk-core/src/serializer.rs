use serde::{ser::SerializeStruct, Serialize};

pub fn to_string_pretty<T>(value: &T) -> serde_json::Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

pub fn to_vec_pretty<T>(value: &T) -> serde_json::Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = serde_json::Serializer::pretty(writer);
    let ser = HexSerializer::new(&mut ser);
    value.serialize(ser)
}

pub struct HexSerializer<S>
where
    S: serde::ser::Serializer,
{
    inner: S,
}

impl<S> HexSerializer<S>
where
    S: serde::ser::Serializer,
{
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<'a, S: 'a> serde::ser::Serializer for HexSerializer<S>
where
    S: serde::ser::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = HexSerializeStruct<S::SerializeStruct>;
    type SerializeStructVariant = S::SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.inner.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.inner.serialize_map(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self.inner.serialize_struct(name, len) {
            Ok(s) => Ok(Self::SerializeStruct::new(s)),
            Err(e) => Err(e),
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}

pub struct HexSerializeStruct<S>
where
    S: SerializeStruct,
{
    inner: S,
}

impl<S> HexSerializeStruct<S>
where
    S: SerializeStruct,
{
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> SerializeStruct for HexSerializeStruct<S>
where
    S: SerializeStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        // Finally, here's the hack. Serialize the value, and try to deserialize
        // it as a Vec<u8>. If that works, hex encode the Vec<u8>, otherwise use
        // the default serialization.
        // Unfortunately there's no way to inspect the generic type parameter T
        // here, otherwise this logic would work by simply checking whether the
        // generic type parameter T is a Vec<u8> or not.
        let as_vec = match serde_json::to_vec(value) {
            Ok(as_vec) => as_vec,
            Err(_) => return self.inner.serialize_field(key, value),
        };
        let val: Vec<u8> = match serde_json::from_slice(&as_vec) {
            Ok(val) => val,
            Err(_) => return self.inner.serialize_field(key, value),
        };
        self.inner.serialize_field(key, &hex::encode(&val))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

pub mod value {
    use std::fmt::Display;

    use serde::{ser::Impossible, Serialize};
    use serde_json::{Error, Map, Result, Value};

    pub fn to_value<T>(value: T) -> Result<Value>
    where
        T: Serialize,
    {
        value.serialize(Serializer)
    }

    pub struct Serializer;

    impl serde::Serializer for Serializer {
        type Ok = Value;
        type Error = Error;

        type SerializeSeq = SerializeVec;
        type SerializeTuple = SerializeVec;
        type SerializeTupleStruct = SerializeVec;
        type SerializeTupleVariant = SerializeTupleVariant;
        type SerializeMap = SerializeMap;
        type SerializeStruct = SerializeMap;
        type SerializeStructVariant = SerializeStructVariant;

        #[inline]
        fn serialize_bool(self, value: bool) -> Result<Value> {
            Ok(Value::Bool(value))
        }

        #[inline]
        fn serialize_i8(self, value: i8) -> Result<Value> {
            self.serialize_i64(value as i64)
        }

        #[inline]
        fn serialize_i16(self, value: i16) -> Result<Value> {
            self.serialize_i64(value as i64)
        }

        #[inline]
        fn serialize_i32(self, value: i32) -> Result<Value> {
            self.serialize_i64(value as i64)
        }

        fn serialize_i64(self, value: i64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }

        fn serialize_i128(self, value: i128) -> Result<Value> {
            if let Ok(value) = i64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(serde::ser::Error::custom("number out of range"))
            }
        }

        #[inline]
        fn serialize_u8(self, value: u8) -> Result<Value> {
            self.serialize_u64(value as u64)
        }

        #[inline]
        fn serialize_u16(self, value: u16) -> Result<Value> {
            self.serialize_u64(value as u64)
        }

        #[inline]
        fn serialize_u32(self, value: u32) -> Result<Value> {
            self.serialize_u64(value as u64)
        }

        #[inline]
        fn serialize_u64(self, value: u64) -> Result<Value> {
            Ok(Value::Number(value.into()))
        }

        fn serialize_u128(self, value: u128) -> Result<Value> {
            if let Ok(value) = u64::try_from(value) {
                Ok(Value::Number(value.into()))
            } else {
                Err(serde::ser::Error::custom("number out of range"))
            }
        }

        #[inline]
        fn serialize_f32(self, float: f32) -> Result<Value> {
            Ok(Value::from(float))
        }

        #[inline]
        fn serialize_f64(self, float: f64) -> Result<Value> {
            Ok(Value::from(float))
        }

        #[inline]
        fn serialize_char(self, value: char) -> Result<Value> {
            let mut s = String::new();
            s.push(value);
            Ok(Value::String(s))
        }

        #[inline]
        fn serialize_str(self, value: &str) -> Result<Value> {
            Ok(Value::String(value.to_owned()))
        }

        fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
            Ok(Value::String(hex::encode(value)))
        }

        #[inline]
        fn serialize_unit(self) -> Result<Value> {
            Ok(Value::Null)
        }

        #[inline]
        fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
            self.serialize_unit()
        }

        #[inline]
        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Value> {
            self.serialize_str(variant)
        }

        #[inline]
        fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Value>
        where
            T: ?Sized + Serialize,
        {
            let mut values = Map::new();
            values.insert(String::from(variant), to_value(value)?);
            Ok(Value::Object(values))
        }

        #[inline]
        fn serialize_none(self) -> Result<Value> {
            self.serialize_unit()
        }

        #[inline]
        fn serialize_some<T>(self, value: &T) -> Result<Value>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
            Ok(SerializeVec {
                vec: Vec::with_capacity(len.unwrap_or(0)),
            })
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct> {
            self.serialize_seq(Some(len))
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant> {
            Ok(SerializeTupleVariant {
                name: String::from(variant),
                vec: Vec::with_capacity(len),
            })
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
            Ok(SerializeMap {
                map: Map::new(),
                next_key: None,
            })
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct> {
            self.serialize_map(Some(len))
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant> {
            Ok(SerializeStructVariant {
                name: String::from(variant),
                map: Map::new(),
            })
        }

        fn collect_str<T>(self, value: &T) -> Result<Value>
        where
            T: ?Sized + Display,
        {
            Ok(Value::String(value.to_string()))
        }
    }

    pub struct SerializeVec {
        vec: Vec<Value>,
    }

    pub struct SerializeTupleVariant {
        name: String,
        vec: Vec<Value>,
    }

    pub struct SerializeMap {
        map: Map<String, Value>,
        next_key: Option<String>,
    }

    pub struct SerializeStructVariant {
        name: String,
        map: Map<String, Value>,
    }

    impl serde::ser::SerializeSeq for SerializeVec {
        type Ok = Value;
        type Error = Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(to_value(value)?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    impl serde::ser::SerializeTuple for SerializeVec {
        type Ok = Value;
        type Error = Error;

        fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            serde::ser::SerializeSeq::serialize_element(self, value)
        }

        fn end(self) -> Result<Value> {
            serde::ser::SerializeSeq::end(self)
        }
    }

    impl serde::ser::SerializeTupleStruct for SerializeVec {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            serde::ser::SerializeSeq::serialize_element(self, value)
        }

        fn end(self) -> Result<Value> {
            serde::ser::SerializeSeq::end(self)
        }
    }

    impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.vec.push(to_value(value)?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();

            object.insert(self.name, Value::Array(self.vec));

            Ok(Value::Object(object))
        }
    }

    impl serde::ser::SerializeMap for SerializeMap {
        type Ok = Value;
        type Error = Error;

        fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.next_key = Some(key.serialize(MapKeySerializer)?);
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            let key = self.next_key.take();
            // Panic because this indicates a bug in the program rather than an
            // expected failure.
            let key = key.expect("serialize_value called before serialize_key");

            // Finally, here's the hack. Serialize the value, and try to deserialize
            // it as a Vec<u8>. If that works, hex encode the Vec<u8>, otherwise use
            // the default serialization.
            // Unfortunately there's no way to inspect the generic type parameter T
            // here, otherwise this logic would work by simply checking whether the
            // generic type parameter T is a Vec<u8> or not.
            let as_vec = match serde_json::to_vec(value) {
                Ok(as_vec) => as_vec,
                Err(_) => {
                    self.map.insert(key, to_value(value)?);
                    return Ok(());
                }
            };
            let val: Vec<u8> = match serde_json::from_slice(&as_vec) {
                Ok(val) => val,
                Err(_) => {
                    self.map.insert(key, to_value(value)?);
                    return Ok(());
                }
            };

            self.map.insert(key, to_value(hex::encode(&val))?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            Ok(Value::Object(self.map))
        }
    }

    impl serde::ser::SerializeStruct for SerializeMap {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            serde::ser::SerializeMap::serialize_entry(self, key, value)
        }

        fn end(self) -> Result<Value> {
            serde::ser::SerializeMap::end(self)
        }
    }

    impl serde::ser::SerializeStructVariant for SerializeStructVariant {
        type Ok = Value;
        type Error = Error;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
        {
            self.map.insert(String::from(key), to_value(value)?);
            Ok(())
        }

        fn end(self) -> Result<Value> {
            let mut object = Map::new();

            object.insert(self.name, Value::Object(self.map));

            Ok(Value::Object(object))
        }
    }

    struct MapKeySerializer;

    fn key_must_be_a_string() -> Error {
        serde::ser::Error::custom("key must be a string")
    }

    fn float_key_must_be_finite() -> Error {
        serde::ser::Error::custom("float key must be finite")
    }

    impl serde::Serializer for MapKeySerializer {
        type Ok = String;
        type Error = Error;

        type SerializeSeq = Impossible<String, Error>;
        type SerializeTuple = Impossible<String, Error>;
        type SerializeTupleStruct = Impossible<String, Error>;
        type SerializeTupleVariant = Impossible<String, Error>;
        type SerializeMap = Impossible<String, Error>;
        type SerializeStruct = Impossible<String, Error>;
        type SerializeStructVariant = Impossible<String, Error>;

        #[inline]
        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<String> {
            Ok(variant.to_owned())
        }

        #[inline]
        fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_bool(self, value: bool) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_i8(self, value: i8) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_i16(self, value: i16) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_i32(self, value: i32) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_i64(self, value: i64) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_i128(self, value: i128) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_u8(self, value: u8) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_u16(self, value: u16) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_u32(self, value: u32) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_u64(self, value: u64) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_u128(self, value: u128) -> Result<String> {
            Ok(value.to_string())
        }

        fn serialize_f32(self, value: f32) -> Result<String> {
            if value.is_finite() {
                Ok(ryu::Buffer::new().format_finite(value).to_owned())
            } else {
                Err(float_key_must_be_finite())
            }
        }

        fn serialize_f64(self, value: f64) -> Result<String> {
            if value.is_finite() {
                Ok(ryu::Buffer::new().format_finite(value).to_owned())
            } else {
                Err(float_key_must_be_finite())
            }
        }

        #[inline]
        fn serialize_char(self, value: char) -> Result<String> {
            Ok({
                let mut s = String::new();
                s.push(value);
                s
            })
        }

        #[inline]
        fn serialize_str(self, value: &str) -> Result<String> {
            Ok(value.to_owned())
        }

        fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_unit(self) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _value: &T,
        ) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            Err(key_must_be_a_string())
        }

        fn serialize_none(self) -> Result<String> {
            Err(key_must_be_a_string())
        }

        fn serialize_some<T>(self, _value: &T) -> Result<String>
        where
            T: ?Sized + Serialize,
        {
            Err(key_must_be_a_string())
        }

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
            Err(key_must_be_a_string())
        }

        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
            Err(key_must_be_a_string())
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleStruct> {
            Err(key_must_be_a_string())
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant> {
            Err(key_must_be_a_string())
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
            Err(key_must_be_a_string())
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct> {
            Err(key_must_be_a_string())
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            _variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant> {
            Err(key_must_be_a_string())
        }

        fn collect_str<T>(self, value: &T) -> Result<String>
        where
            T: ?Sized + Display,
        {
            Ok(value.to_string())
        }
    }
}
