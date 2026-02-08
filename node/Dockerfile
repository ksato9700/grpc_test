FROM node:22-alpine AS builder
WORKDIR /app
RUN npm install -g pnpm
COPY package.json pnpm-lock.yaml* ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm build

FROM node:22-alpine
WORKDIR /app
RUN npm install -g pnpm
COPY package.json pnpm-lock.yaml* ./
# Install only production dependencies
RUN pnpm install --prod --frozen-lockfile

# Copy compiled files and required proto definitions
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/proto ./proto

ENV PORT=8080
EXPOSE 8080
# Use node directly for production
CMD ["node", "dist/server.js"]
