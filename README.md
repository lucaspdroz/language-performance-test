# language-performance-test

## Pre-requisites

- docker (28.4.0)
- go (1.23.3)
- java
- node (v20.18.1)
- dotnet

For env I used `"openssl rand -base64 32"`, as .env key

## Starting the enviroment

create the hole database enviroment for each test

> docker compose up -d

### Testing requests

> curl -X POST http://localhost:8080/encode -H 'Content-Type: application/json' -d '{"date":"2025-10-01","time":"20:30:00","batch":"LTC-123"}'

retorna {"id":"..."}

### decode
>
> curl http://localhost:8080/decode/id

### GO

Don't forget to add the Go into your PATH

> go mod init api_go

--- don't run ---
> go install github.com/swaggo/swag/cmd/swag@latest

--- Run ---
> go get github.com/gin-gonic/gin@v1.10.0
> go get gorm.io/gorm@v1.25.7
> go get gorm.io/driver/postgres@v1.5.9
> go get github.com/swaggo/gin-swagger@v1.6.7
> go get github.com/swaggo/files@v0.2.0
--- install the mod ---
> go mod tidy
--- run api ---
> go run main.go

--- generate swagger ---
> swag init
