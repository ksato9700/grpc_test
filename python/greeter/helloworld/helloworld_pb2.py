# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: helloworld.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from . import blood_type_pb2 as blood__type__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x10helloworld.proto\x12\nhelloworld\x1a\x10\x62lood_type.proto\"2\n\x05\x45xtra\x12\x15\n\rextra_message\x18\x01 \x01(\t\x12\x12\n\nextra_code\x18\x02 \x01(\x03\"\x84\x01\n\x0cHelloRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0b\n\x03ver\x18\x02 \x01(\x03\x12(\n\tbloodType\x18\x03 \x01(\x0e\x32\x15.helloworld.BloodType\x12%\n\x05\x65xtra\x18\x04 \x01(\x0b\x32\x11.helloworld.ExtraH\x00\x88\x01\x01\x42\x08\n\x06_extra\"\x1d\n\nHelloReply\x12\x0f\n\x07message\x18\x01 \x01(\t2I\n\x07Greeter\x12>\n\x08sayHello\x12\x18.helloworld.HelloRequest\x1a\x16.helloworld.HelloReply\"\x00\x42\x1f\n\nhelloworldB\x0fHelloWorldProtoP\x01\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'helloworld_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  DESCRIPTOR._serialized_options = b'\n\nhelloworldB\017HelloWorldProtoP\001'
  _globals['_EXTRA']._serialized_start=50
  _globals['_EXTRA']._serialized_end=100
  _globals['_HELLOREQUEST']._serialized_start=103
  _globals['_HELLOREQUEST']._serialized_end=235
  _globals['_HELLOREPLY']._serialized_start=237
  _globals['_HELLOREPLY']._serialized_end=266
  _globals['_GREETER']._serialized_start=268
  _globals['_GREETER']._serialized_end=341
# @@protoc_insertion_point(module_scope)
