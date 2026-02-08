import * as grpc from '@grpc/grpc-js';
import { helloworld } from './proto.js';

const sayHello: grpc.UntypedHandleCall = (call, callback) => {
  const name = call.request.name;
  console.log(`Received request from: ${name}`);

  if (name === 'Donald') {
    return callback({
      code: grpc.status.INVALID_ARGUMENT,
      details: `Ouch! I don't like you, ${name}`,
    });
  }

  const message = `Hello ${name}!`;
  callback(null, { message });
};

function main() {
  const server = new grpc.Server();
  server.addService(helloworld.Greeter.service, { sayHello });

  const port = process.env.PORT || '50051';
  const address = `0.0.0.0:${port}`;

  server.bindAsync(address, grpc.ServerCredentials.createInsecure(), (err, boundPort) => {
    if (err) {
      console.error(`Failed to bind server: ${err.message}`);
      return;
    }
    console.log(`gRPC server running at http://0.0.0.0:${boundPort}`);
  });
}

main();
