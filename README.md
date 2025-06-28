![cdlogo](https://carefuldata.com/images/cdlogo.png)

# morpho-web

A rust template for front-end web server microservice container using actix web framework.

This version of morpho is now archived, only the README will be updated. Newer versions of crates have since come out that result in different needs. Rather than updating this one, it is being preserved for use or reference while the newer template is being refined and tested.

Links will be added to this README for the new version/s, likely within the next few months.


## rustls for HTTPS

This program uses rustls for TLS, leveraging the strong defaults. It includes support for TLSv1.2 and TLSv1.3 only,
with weak features disabled. The downside of rustls is that it doesn't have a wide of a range of feature support as openssl,
although is very correct and performant and should ideally be adopted when possible.

## tokio async io

We can serve a lot of requests with actix use of tokio async io, letting IO-bound workloads scale very well.
The reading of files from the filesystem is not special in terms of performance, peforming much like other
web servers. The performance is very good and reliable.

## cloud native design

This web server template is cloud native, working well in Kubernetes and Docker, etc.
It works well with many replicas, has a minimized set of dependencies and libraries,
and puts security as a priority.

## redirecting to HTTPS

Port redirection is included by default now.

## security headers

HSTS and security headers are inserted by default.


### Find more templates based on morpho-web:

#### https://github.com/jpegleg/callsoup for a morpho based service that includes reqwest callbacks and redis storage

#### https://github.com/jpegleg/morpho-sessions for a morpho based service template that includes cookies

#### https://github.com/jpegleg/flying-squirrel-tactix postgresql manager with rustls

#### https://github.com/jpegleg/squirrel-tactix postgresql manager without TLS

#### https://github.com/jpegleg/morpho-web2 for openssl instead of rustls 

#### https://github.com/jpegleg/morpho-web-lt for stripped down version without logging


### Some notes about container builds

The alternate build Dockerfile_ekidd uses the `FROM ekidd/rust-musl-builder AS build` to compile with cargo
and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI
image has no shell, nothing but the dependencies for the web server.

11/27/22 - the ekidd builder has some issues when compiling these dependencies, so we have switched the builder to clux/muslrust:stable
With this change, we are also choosing to separate the compiling and the docker image build into a two step process for more granular
CI actions and artifact creation.

```
docker run -v $PWD:/volume --rm -t clux/muslrust:stable cargo build --release
docker build -t "localhost:5000/morpho-web" .
```

01/15/24 - Cargo "cross" is sufficient for musl compiling, there is no need for the ekidd builder or clux/muslrust. Cross has way more build targets
and I have been using it instead for some time.

```
cross build --target x86_64-unknown-linux-musl --release
docker build -t "localhost:5000/morpho-web" .
```

The base image is less than 15MB for the entire framework. The size of the added content from `static`
will increase the image size etc. Alternatively to doing a copy into the container image,
the /app/static directory can be a volume mount containing the content to load. Note that by default the cert and key pair are in /app/ which is the workdir for the server, while the webroot is /app/static/.

From the test docker-compose.yml:

```
    volumes:
      - /opt/protean-gitops/static/:/app/static/
      - /opt/protean-gitops/cert.pem:/app/cert.pem
      - /opt/protean-gitops/privkey.pem:/app/privkey.pem
```

Kubernetes manifests for morpho-web are not included in this repo.
