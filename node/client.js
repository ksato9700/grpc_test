const grpc = require('grpc');
const helloworld = require('./helloworld');

const client = new helloworld.Greeter('127.0.0.1:50051', grpc.credentials.createInsecure())

client.sayHello({ name: 'Node', ver: 321 }, (error, response) => {
    if (!error) {
        console.log(response.message);
    } else {
        console.error(error)
    }
})
