package helloworld;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.fail;

import io.grpc.ManagedChannel;
import io.grpc.Status;
import io.grpc.StatusRuntimeException;
import io.grpc.inprocess.InProcessChannelBuilder;
import io.grpc.inprocess.InProcessServerBuilder;
import io.grpc.testing.GrpcCleanupRule;
import org.junit.Before;
import org.junit.Rule;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

@RunWith(JUnit4.class)
public class HelloWorldClientTest {
    @Rule public final GrpcCleanupRule grpcCleanup = new GrpcCleanupRule();

    private HelloWorldClient client;

    @Before
    public void setUp() throws Exception {
        String serverName = InProcessServerBuilder.generateName();
        grpcCleanup.register(
                InProcessServerBuilder.forName(serverName)
                        .directExecutor()
                        .addService(new HelloWorldServer.GreeterImpl())
                        .build()
                        .start());
        ManagedChannel channel =
                grpcCleanup.register(
                        InProcessChannelBuilder.forName(serverName).directExecutor().build());
        client = new HelloWorldClient(channel);
    }

    @Test
    public void greet_success() {
        client.greet("Ken");
    }

    @Test
    public void sayHello_donald() {
        String serverName = InProcessServerBuilder.generateName();
        try {
            grpcCleanup.register(
                    InProcessServerBuilder.forName(serverName)
                            .directExecutor()
                            .addService(new HelloWorldServer.GreeterImpl())
                            .build()
                            .start());
        } catch (Exception e) {
            fail(e.getMessage());
        }
        ManagedChannel channel =
                grpcCleanup.register(
                        InProcessChannelBuilder.forName(serverName).directExecutor().build());
        GreeterGrpc.GreeterBlockingStub stub = GreeterGrpc.newBlockingStub(channel);

        HelloRequest request = HelloRequest.newBuilder().setName("Donald").build();
        try {
            stub.sayHello(request);
            fail("Should have thrown StatusRuntimeException");
        } catch (StatusRuntimeException e) {
            assertEquals(Status.Code.INVALID_ARGUMENT, e.getStatus().getCode());
            assertEquals("Ouch! I don't like you, Donald", e.getStatus().getDescription());
        }
    }
}
