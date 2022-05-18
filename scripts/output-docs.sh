#!/bin/bash
cargo doc --no-deps
if [ ! -d ./docs ]; then
  mkdir ./docs
fi
cp -r ./target/doc/* ./docs/
echo "<meta http-equiv=\"refresh\" content=\"0; url=insider_bot_rs\">" > ./docs/index.html
