FROM golang:1.23.4-alpine AS builder

RUN apk add --no-cache git
WORKDIR /app
COPY . .
RUN go mod download

RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o /go/bin/app ./cmd/server/main.go

FROM alpine:3.19
RUN adduser -D -g '' appuser
COPY --from=builder /go/bin/app /app
RUN chown appuser:appuser /app
USER appuser

ENTRYPOINT ["/app"]
