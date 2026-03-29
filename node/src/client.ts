import * as grpc from '@grpc/grpc-js';
import type { HelloReply } from './proto.js';
import { helloworld } from './proto.js';

async function main() {
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
    await new Promise<void>((resolve) => {
      client.sayHello({ name }, (error: grpc.ServiceError | null, response: HelloReply) => {
        if (!error) {
          console.log(`Greeting: ${response.message}`);
        } else {
          console.error(`RPC failed for ${name}: ${error.message}`);
        }
        resolve();
      });
    });
  }
}

main();
