# profileforge

## Generate a PNG with Docker

Build the image and generate a PNG:

```sh
docker build -t profileforge .
mkdir -p docker-output
docker run --rm --user "$(id -u):$(id -g)" -e HOME=/tmp \
  -v "$(pwd):/input:ro" \
  -v "$(pwd)/docker-output:/output" \
  profileforge /input/examples/sample.yaml /output/profile.png
```

The output is generated at `docker-output/profile.png`. The container runs as your current user, so no permission changes are needed. Docker's default seccomp profile remains enabled. This image disables Chromium's own sandbox because some Docker hosts restrict the namespaces it needs; the container still runs as a non-root user, and the input is mounted read-only.
