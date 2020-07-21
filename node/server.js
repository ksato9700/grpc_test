const { grpc, apis } = require('apis');

const server = new grpc.Server();

const sayHello = (call, callback) => {
    const message = `Hello ${call.request.name}!`;
    console.log('sayHello');
    console.log('request: ', call.request);
    console.log('response:', message);
    callback(null, { message });
}

server.addService(apis.Greeter.service, {
    sayHello
});

server.bind('127.0.0.1:50051', grpc.ServerCredentials.createInsecure());
console.log('gRPC server running at http://127.0.0.1:50051');
server.start();
