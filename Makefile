.DEFAULT: build run

tag = 0.0.1-alpha.1

build:
	docker build --tag loggy-server-rs:$(tag) .

deploy:
	docker buildx build --push --platform linux/arm64 --tag bloveless/loggy-server-rs:$(tag) .

clean:
	docker container stop loggy-server
	docker container rm loggy-server

run:
	docker run --name loggy-server loggy-server-rs:$(tag)

exec:
	docker exec -it loggy-server bash

shell:
	docker run --rm -it --entrypoint bash loggy-server-rs:$(tag)
