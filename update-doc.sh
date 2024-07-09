#!/bin/bash

grep '^\[!' README.md >README.md.tmp
echo >>README.md.tmp
sed -n '/pub mod extensions;/q;p' src/lib.rs | sed 's/^\/\/\/ \?//g' | grep -v '# let' | grep -v '^#!' >>README.md.tmp
echo >>README.md.tmp
sed -n '/## Inspiration/,$ p' README.md >>README.md.tmp
mv -f README.md.tmp README.md

