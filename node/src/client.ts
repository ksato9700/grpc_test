import * as grpc from '@grpc/grpc-js';
import { helloworld } from './proto.js';

function main() {
  const args = process.argv.slice(2);
  let target = 'localhost:50051';
  let names: string[] = [];

  if (args.length > 0 && args[0].includes(':')) {
    target = args[0];
    names = args.slice(1);
  } else {
    names = args;
  }

  if (names.length === 0) {
    names = ['Node'];
  }

  const client = new helloworld.Greeter(target, grpc.credentials.createInsecure());

  for (const name of names) {
    client.sayHello({ name }, (error: grpc.ServiceError | null, response: any) => {
      if (!error) {
        console.log(`Greeting: ${response.message}`);
      } else {
        console.error(`RPC failed for ${name}: ${error.message}`);
      }
    });
  }
}

main();
