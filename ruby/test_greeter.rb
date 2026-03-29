require "minitest/autorun"
require "grpc"

this_dir = __dir__
lib_dir = File.join(this_dir, "lib")
$LOAD_PATH.unshift(lib_dir) unless $LOAD_PATH.include?(lib_dir)

require "helloworld_services_pb"
require_relative "server"

class GreeterTest < Minitest::Test
  def setup
    @rpc_server = GRPC::RpcServer.new
    @port = @rpc_server.add_http2_port("localhost:0", :this_port_is_insecure)
    @rpc_server.handle(GreeterServer)
    @thread = Thread.new { @rpc_server.run }
    @rpc_server.wait_till_running
    @stub = Helloworld::Greeter::Stub.new("localhost:#{@port}", :this_channel_is_insecure)
  end

  def teardown
    @rpc_server.stop
    @thread.join
  end

  def test_say_hello
    response = @stub.say_hello(Helloworld::HelloRequest.new(name: "Ruby"))
    assert_equal "Hello Ruby!", response.message
  end

  def test_say_hello_donald
    error = assert_raises(GRPC::InvalidArgument) do
      @stub.say_hello(Helloworld::HelloRequest.new(name: "Donald"))
    end
    assert_match "I don't like you, Donald", error.message
  end
end
