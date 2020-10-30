proto_files = \
	proto/blood_type.proto \
	proto/helloworld.proto

$(proto_files): protodep.toml
	protodep up -f -c -i unused
