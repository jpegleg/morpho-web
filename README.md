![net-gargoyle2](https://carefuldata.com/images/cdlogo.png)

# morpho-web

A rust template for front-end web server microservice container using actix web framework.

The included Dockerfile uses the `FROM ekidd/rust-musl-builder AS build` to compile with cargo
and then we copy the dependencies into a `FROM scratch` empty container. The resulting OCI
image has no shell, nothing but the dependencies for the web server.

The base image is less than 12MB for the entire framework. The size of the added content from `static`
will increase the image size etc. Alternatively to doing a copy into the container image,
the /app/static directory can be a volume mount containing the content to load. Note that by default the cert and key pair are in /app/ which is the workdir for the server, while the webroot is /app/static/.

From the test docker-compose.yml:

```
    volumes:
      - /opt/protean-gitops/static/:/app/static/
      - /opt/protean-gitops/cert.pem:/app/cert.pem
      - /opt/protean-gitops/privkey.pem:/app/privkey.pem
```

In production, rather than using Docker, we can use Kubernetes and mount those more appropriately.
The purpose of the docker-compose.yml and the protean references are for some testing systems usage.

## rustls for HTTPS

This program uses rustls for TLS, leveraging the strong defaults. It includes support for TLSv1.2 and TLSv1.3 only,
with weak features disabled.

## tokio async io

We can serve a lot of requests with actix use of tokio async io, letting IO-bound workloads scale very well.
The reading of files from the filesystem is not special in terms of performance, peforming much like other
web servers. The performance is very good and reliable.

## cloud native design

This web server template is cloud native, working well in Kubernetes and Docker, etc.
It works well with many replicas, has a minimized set of dependencies and libraries,
and puts security as a priority.

## redirecting port 80

As of the current version, morpho-web does not redirect port 80 to 443. This type of redirection is to be handled "in front" of the morpho-server for now.

## header formatting warning

A downside of this software currently is that the headers get set to lowercase which is causing various checks and detection mechanisms to fail.

