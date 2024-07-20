#!/usr/bin/env bash
# Update list of methods in the documentation
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)"
cd ${SCRIPT_DIR}/..

grep '^\[!' README.md >README.md.tmp
echo >>README.md.tmp
sed -n '/^pub.*/q;p' src/lib.rs | grep -v '# let' | grep -v '^#!' | sed 's/^\/\/! \?//' | sed 's/\[`/[/' | sed 's/`\](/](https:\/\/docs.rs\/cantrip\/latest\/cantrip\/trait./' | sed 's/::/.html#method./' | sed 's/\.html#method\.from/::from/' | sed 's/\.html#method\.\*/::*/' >>README.md.tmp
echo -e "\n" >>README.md.tmp
sed -n '/## Inspiration/,$ p' README.md >>README.md.tmp
mv -f README.md.tmp README.md

