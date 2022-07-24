import * as grpc from '@grpc/grpc-js';
import * as assert from 'assert';
import * as helloworld from './helloworld.js';

const server = new grpc.Server();

const sayHello = (call, callback) => {
    const message = `Hello ${call.request.name}!`;
    console.log('sayHello');
    console.log('request: ', call.request);
    console.log('response:', message);
    callback(null, { message });
}
// console.log(helloworld.service, { sayHello });

server.addService(helloworld.service, { sayHello });

const port = process.env.PORT || "50051"
const server_address = `[::]:${port}`;
server.bindAsync(
    server_address,
    grpc.ServerCredentials.createInsecure(),
    (err, port) => {
        assert.ifError(err)
        console.log(port);
        console.log(`gRPC server running at ${server_address}`);
        server.start();
    }
);
