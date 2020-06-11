# BUILD
from golang:1.14 as build-env

WORKDIR /ad-server

COPY ./ad-server/ /ad-server/
RUN go mod download -x
RUN go build -o /go/bin/ad-server main.go index.go

# RUNTIME
FROM gcr.io/distroless/base

# Copy executable to runtime
COPY --from=build-env /go/bin/ad-server /

# Copy resources to runtime
COPY --from=build-env /ad-server/images /images
COPY --from=build-env /ad-server/js /js

ENTRYPOINT ["/ad-server"]