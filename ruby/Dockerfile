FROM ruby:3.3-slim-bookworm AS builder

RUN apt-get update -qq && apt-get install -y build-essential

WORKDIR /app
COPY Gemfile ./
RUN bundle install

FROM ruby:3.3-slim-bookworm

WORKDIR /app
COPY --from=builder /usr/local/bundle /usr/local/bundle
COPY lib ./lib
COPY Gemfile server.rb client.rb ./

ENV PORT=8080
EXPOSE 8080
CMD ["bundle", "exec", "ruby", "server.rb"]
