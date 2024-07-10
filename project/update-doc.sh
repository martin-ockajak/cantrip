#!/usr/bin/env bash
# Update list of methods in the documentation
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)"
cd ${SCRIPT_DIR}/..

grep '^\[!' README.md >README.md.tmp
echo >>README.md.tmp
sed -n '/pub mod extensions;/q;p' src/lib.rs | sed 's/^\/\/\/ \?//g' | grep -v '# let' | grep -v '^#!' >>README.md.tmp
echo >>README.md.tmp
sed -n '/## Inspiration/,$ p' README.md >>README.md.tmp
mv -f README.md.tmp README.md

