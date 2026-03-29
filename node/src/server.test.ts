import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import * as grpc from '@grpc/grpc-js';
import { helloworld } from './proto.js';
import { createServer } from './server.js';

describe('Greeter', () => {
  let server: grpc.Server;
  let client: any;

  beforeAll(async () => {
    server = createServer();
    const port = await new Promise<number>((resolve, reject) => {
      server.bindAsync('localhost:0', grpc.ServerCredentials.createInsecure(), (err, p) => {
        if (err) reject(err);
        else resolve(p);
      });
    });
    client = new helloworld.Greeter(`localhost:${port}`, grpc.credentials.createInsecure());
  });

  afterAll(() => {
    client.close();
    server.forceShutdown();
  });

  it('returns a greeting for a valid name', async () => {
    const response = await new Promise<{ message: string }>((resolve, reject) => {
      client.sayHello({ name: 'Node' }, (err: grpc.ServiceError | null, res: { message: string }) => {
        if (err) reject(err);
        else resolve(res);
      });
    });
    expect(response.message).toBe('Hello Node!');
  });

  it('rejects Donald with INVALID_ARGUMENT', async () => {
    const error = await new Promise<grpc.ServiceError>((resolve, reject) => {
      client.sayHello({ name: 'Donald' }, (err: grpc.ServiceError | null) => {
        if (err) resolve(err);
        else reject(new Error('Expected an error but call succeeded'));
      });
    });
    expect(error.code).toBe(grpc.status.INVALID_ARGUMENT);
  });
});
