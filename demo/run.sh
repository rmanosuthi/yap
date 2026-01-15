#!/bin/sh

set -eux

prepare() {
	rm -rf deploy/
	mkdir deploy/
	cp -f server.json deploy/
	podman-compose build
}

db() {
	podman-compose up db
}

server() {
	podman-compose run server
}

register() {
	podman-compose run client /app/target/release/client register \
		--cfg-path "/deploy/client_${1}.json" --addr http://server:8080 \
		--email "client_${1}@demo" --password "${2}"
}

client() {
	podman-compose run client /app/target/release/client login \
		--cfg-path "/deploy/client_${1}.json"
}

case "$1" in
-h|--help) echo "see README.md";;
prepare) prepare;;
db) db;;
server) server;;
register) register "$2" "$3";;
client) client "$2";;
*) echo "bad argument" && break;;
esac
