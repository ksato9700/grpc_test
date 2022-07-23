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

server.bindAsync(
    '127.0.0.1:50051',
    grpc.ServerCredentials.createInsecure(),
    (err, _port) => {
        assert.ifError(err)
        console.log('gRPC server running at http://127.0.0.1:50051');
        server.start();
    }
);
