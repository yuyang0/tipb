// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct TableCheckSumReq {
    // message fields
    tp: ::std::option::Option<AlgorithmType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TableCheckSumReq {}

impl TableCheckSumReq {
    pub fn new() -> TableCheckSumReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TableCheckSumReq {
        static mut instance: ::protobuf::lazy::Lazy<TableCheckSumReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TableCheckSumReq,
        };
        unsafe {
            instance.get(TableCheckSumReq::new)
        }
    }

    // optional .tipb.AlgorithmType tp = 1;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: AlgorithmType) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp(&self) -> AlgorithmType {
        self.tp.unwrap_or(AlgorithmType::TypeCrc32Xor)
    }

    fn get_tp_for_reflect(&self) -> &::std::option::Option<AlgorithmType> {
        &self.tp
    }

    fn mut_tp_for_reflect(&mut self) -> &mut ::std::option::Option<AlgorithmType> {
        &mut self.tp
    }
}

impl ::protobuf::Message for TableCheckSumReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.tp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tp {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tp {
            os.write_enum(1, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TableCheckSumReq {
    fn new() -> TableCheckSumReq {
        TableCheckSumReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<TableCheckSumReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AlgorithmType>>(
                    "tp",
                    TableCheckSumReq::get_tp_for_reflect,
                    TableCheckSumReq::mut_tp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TableCheckSumReq>(
                    "TableCheckSumReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TableCheckSumReq {
    fn clear(&mut self) {
        self.clear_tp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TableCheckSumReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TableCheckSumReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AlgorithmType {
    TypeCrc32Xor = 0,
}

impl ::protobuf::ProtobufEnum for AlgorithmType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AlgorithmType> {
        match value {
            0 => ::std::option::Option::Some(AlgorithmType::TypeCrc32Xor),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AlgorithmType] = &[
            AlgorithmType::TypeCrc32Xor,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AlgorithmType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AlgorithmType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AlgorithmType {
}

impl ::protobuf::reflect::ProtobufValue for AlgorithmType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0echecksum.proto\x12\x04tipb\x1a\x14gogoproto/gogo.proto\"9\n\x10Tab\
    leCheckSumReq\x12%\n\x02tp\x18\x01\x20\x01(\x0e2\x13.tipb.AlgorithmTypeB\
    \x04\xc8\xde\x1f\0*!\n\rAlgorithmType\x12\x10\n\x0cTypeCrc32Xor\x10\0B%\
    \n\x15com.pingcap.tidb.tipbP\x01\xe0\xe2\x1e\x01\xc8\xe2\x1e\x01\xd0\xe2\
    \x1e\x01J\xc9\x05\n\x06\x12\x04\0\0\x13\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0c\n\x08\n\x01\x08\x12\x03\x04\0\"\
    \n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x04\0\"\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x04\x07\x1a\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x04\x1d!\n\x08\n\x01\x08\x12\x03\x05\0.\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\x05\0.\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\x05\x07\x13\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x13\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x05\x07\x13\n\x0c\n\x05\x08\
    \xe7\x07\x01\x07\x12\x03\x05\x16-\n\t\n\x02\x03\0\x12\x03\x07\x07\x1d\n\
    \x08\n\x01\x08\x12\x03\t\0(\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\t\0(\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\t\x07\x20\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\t\x07\x20\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\t\x08\x1f\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\t#'\n\x08\n\x01\
    \x08\x12\x03\n\0$\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\n\0$\n\x0c\n\x05\
    \x08\xe7\x07\x03\x02\x12\x03\n\x07\x1c\n\r\n\x06\x08\xe7\x07\x03\x02\0\
    \x12\x03\n\x07\x1c\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\n\x08\
    \x1b\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\n\x1f#\n\x08\n\x01\x08\x12\
    \x03\x0b\0*\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x0b\0*\n\x0c\n\x05\x08\
    \xe7\x07\x04\x02\x12\x03\x0b\x07\"\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\
    \x03\x0b\x07\"\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\x0b\x08!\n\
    \x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03\x0b%)\n\n\n\x02\x05\0\x12\x04\r\0\
    \x0f\x01\n\n\n\x03\x05\0\x01\x12\x03\r\x05\x12\n\x0b\n\x04\x05\0\x02\0\
    \x12\x03\x0e\x02\x13\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x0e\x02\x0e\n\
    \x0c\n\x05\x05\0\x02\0\x02\x12\x03\x0e\x11\x12\n\n\n\x02\x04\0\x12\x04\
    \x11\0\x13\x01\n\n\n\x03\x04\0\x01\x12\x03\x11\x08\x18\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x12\x02?\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x12\x02\n\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03\x12\x0b\x18\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x12\x19\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x12\x1e\x1f\
    \n\x0c\n\x05\x04\0\x02\0\x08\x12\x03\x12\x20>\n\x0f\n\x08\x04\0\x02\0\
    \x08\xe7\x07\0\x12\x03\x12!=\n\x10\n\t\x04\0\x02\0\x08\xe7\x07\0\x02\x12\
    \x03\x12!5\n\x11\n\n\x04\0\x02\0\x08\xe7\x07\0\x02\0\x12\x03\x12!5\n\x12\
    \n\x0b\x04\0\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x03\x12\"4\n\x10\n\t\x04\
    \0\x02\0\x08\xe7\x07\0\x03\x12\x03\x128=\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
