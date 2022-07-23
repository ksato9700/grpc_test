import * as grpc from '@grpc/grpc-js';
import * as helloworld from './helloworld.js';

const client = new helloworld.server.Greeter('127.0.0.1:50051', grpc.credentials.createInsecure())

client.sayHello({ name: 'Node', ver: 321 }, (error, response) => {
    if (!error) {
        console.log(response.message);
    } else {
        console.error(error)
    }
})
