# profileforge

## Generate a PNG with Docker

Build the image and export a PNG to your working directory.

```sh
docker build -t profileforge .
mkdir -p docker-output
chmod 0777 docker-output
docker run --rm --security-opt seccomp=unconfined \
	-v "$(pwd):/input:ro" \
	-v "$(pwd)/docker-output:/output" \
	profileforge /input/examples/sample.yaml /output/profile.png
```

The output is generated at `docker-output/profile.png`. Use `chmod 0777` only for the output directory. `seccomp=unconfined` disables Docker's seccomp restriction, so it should be used only with trusted input.