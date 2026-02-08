/*
 * Copyright 2015 The gRPC Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package helloworld;

import io.grpc.Channel;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.StatusRuntimeException;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.logging.Logger;

public class HelloWorldClient {
    private static final Logger logger = Logger.getLogger(HelloWorldClient.class.getName());

    private final GreeterGrpc.GreeterBlockingStub blockingStub;

    public HelloWorldClient(Channel channel) {
        blockingStub = GreeterGrpc.newBlockingStub(channel);
    }

    /** Say hello to server. */
    public void greet(String name) {
        HelloRequest.Builder requestBuilder =
                HelloRequest.newBuilder().setName(name).setVer(123).setBloodType(BloodType.B);

        if ("Bill".equals(name)) {
            requestBuilder.setExtra(
                    Extra.newBuilder().setExtraMessage("hey there").setExtraCode(123).build());
        }

        HelloReply response;
        try {
            response = blockingStub.sayHello(requestBuilder.build());
        } catch (StatusRuntimeException e) {
            logger.log(Level.WARNING, "RPC failed for {0}: {1}", new Object[] {name, e.getStatus()});
            return;
        }
        logger.info("Greeting: " + response.getMessage());
    }

    public static void main(String[] args) throws Exception {
        String target = "localhost:50051";
        int startIndex = 0;

        if (args.length > 0 && args[0].contains(":")) {
            target = args[0];
            startIndex = 1;
        }

        ManagedChannel channel = ManagedChannelBuilder.forTarget(target).usePlaintext().build();
        try {
            HelloWorldClient client = new HelloWorldClient(channel);
            if (args.length > startIndex) {
                for (int i = startIndex; i < args.length; i++) {
                    client.greet(args[i]);
                }
            } else {
                client.greet("Ken");
            }
        } finally {
            channel.shutdownNow().awaitTermination(5, TimeUnit.SECONDS);
        }
    }
}
