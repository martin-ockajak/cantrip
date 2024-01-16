#!/bin/bash

sed -n '/pub mod extensions;/q;p' src/lib.rs | sed 's/^\/\/\/ \?//g' >README.md.tmp
sed -n '/## Inspiration/,$ p' README.md >>README.md.tmp
mv -f README.md.tmp README.md

