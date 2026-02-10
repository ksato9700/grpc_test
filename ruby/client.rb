#!/usr/bin/env ruby

this_dir = __dir__
lib_dir = File.join(this_dir, "lib")
$LOAD_PATH.unshift(lib_dir) unless $LOAD_PATH.include?(lib_dir)

require "grpc"
require "helloworld_services_pb"

def main
  args = ARGV.dup
  target = "localhost:50051"

  if args.length > 0 && args[0].include?(":")
    target = args.shift
  end

  names = args
  names = ["Ruby"] if names.empty?

  stub = Helloworld::Greeter::Stub.new(target, :this_channel_is_insecure)

  names.each do |name|
    message = stub.say_hello(Helloworld::HelloRequest.new(name: name)).message
    puts "Greeting: #{message}"
  rescue GRPC::BadStatus => e
    puts "RPC failed for #{name}: #{e.message}"
  end
end

main
