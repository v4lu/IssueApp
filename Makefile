dev-containers:
	@docker-compose -f docker-compose.dev.yml up -d

docs-gh-api:
	@ cd gh-api && swag init -g cmd/server/main.go && cd ..

start-gh-api:
	@cd gh-api && go run cmd/server/main.go
