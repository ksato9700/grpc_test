PROTOS_DIR=../proto
OUTDIR=lib

gen-grpc:
	mkdir -p $(OUTDIR)
	bundle exec grpc_tools_ruby_protoc -I $(PROTOS_DIR) --ruby_out=$(OUTDIR) --grpc_out=$(OUTDIR) $(PROTOS_DIR)/*.proto

build:
	rm -rf vendor/bundle .bundle Gemfile.lock
	mkdir -p $(OUTDIR)
	bundle install --path vendor/bundle
	# Strip BUNDLED WITH section to avoid version mismatch with older bundler
	sed -i '' '/BUNDLED WITH/,+1d' Gemfile.lock
	bundle exec grpc_tools_ruby_protoc -I $(PROTOS_DIR) --ruby_out=$(OUTDIR) --grpc_out=$(OUTDIR) $(PROTOS_DIR)/*.proto

protos: gen-grpc

build-docker: protos
	docker build . -t greeter-server-ruby:latest

run-server:
	PORT=50051 bundle exec ruby server.rb

run-server-docker:
	docker run -it --rm -e PORT=8080 -p 50051:8080 greeter-server-ruby:latest

run-client:
	bundle exec ruby client.rb $(ARGS)
