# BUILD
from golang:1.14 as build-env

WORKDIR /internal-service

COPY ./internal-service/ /internal-service/
RUN go build -o /go/bin/internal-service main.go

# RUNTIME
FROM gcr.io/distroless/base

# Copy executable to runtime
WORKDIR /internal-service
COPY --from=build-env /go/bin/internal-service ./internal-service
COPY --from=build-env /internal-service/index.html ./index.html


ENTRYPOINT ["./internal-service"]