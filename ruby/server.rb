#!/usr/bin/env ruby

this_dir = __dir__
lib_dir = File.join(this_dir, "lib")
$LOAD_PATH.unshift(lib_dir) unless $LOAD_PATH.include?(lib_dir)

require "grpc"
require "helloworld_services_pb"

class GreeterServer < Helloworld::Greeter::Service
  def say_hello(hello_req, _unused_call)
    name = hello_req.name
    puts "Received request for: #{name}"

    if name == "Donald"
      raise GRPC::InvalidArgument.new("Ouch! I don't like you, #{name}")
    end

    Helloworld::HelloReply.new(message: "Hello #{name}!")
  end
end

def main
  port = ENV["PORT"] || "50051"
  s = GRPC::RpcServer.new
  s.add_http2_port("0.0.0.0:#{port}", :this_port_is_insecure)
  puts "gRPC server running at http://0.0.0.0:#{port}"
  s.handle(GreeterServer)
  s.run_till_terminated_or_interrupted([1, "int", "SIGQUIT"])
end

main
