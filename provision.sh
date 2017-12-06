#!/bin/sh

sudo apt install build-essential --yes

curl --silent --output /tmp/rustup.sh https://static.rust-lang.org/rustup.sh
chmod +x /tmp/rustup.sh
/tmp/rustup.sh --revision=1.7.0 > /dev/null 2>&1
