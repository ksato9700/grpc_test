const { grpc, apis } = require('apis');

const client = new apis.Greeter('127.0.0.1:50051', grpc.credentials.createInsecure())

client.sayHello({ name: 'Node', ver: 321 }, (error, response) => {
    if (!error) {
        console.log(response.message);
    } else {
        console.error(error)
    }
})
