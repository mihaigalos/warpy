_default:
  @just --list --unsorted

ip := "127.0.0.1"
port := "8082"

_start:
    cargo run --example simple &
    sleep 10

_stop:
    pkill simple

build:
    cargo build

test: build _start && _stop
    #!/bin/bash
    function err() {
        echo -e "\e[1;31m${@}\e[0m" >&2
        exit 1
    }

    function ok() {
        echo -e "\e[1;32mOK\e[0m"
    }

    sha_expected=c87fc1505070fe84c9c9f745b303d6cabd9cacf8e2aa65ddd854d1b81d4c8a72

    pushd $(mktemp -d)
    wget {{ ip }}:{{ port }}/test/demofile
    sha_actual=$(sha256sum demofile | cut -d' ' -f1)
    [ "$sha_actual" = "$sha_expected" ] && ok || err "ERROR: input and output SHA256s don't match."
    popd
