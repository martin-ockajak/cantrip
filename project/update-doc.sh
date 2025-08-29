#!/usr/bin/env bash
# Update list of methods in the documentation
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)"
cd ${SCRIPT_DIR}/..

sed -n '/^A Swiss/q;p' README.md >README.md.tmp
sed -n '/^pub.*/q;p' src/lib.rs \
  | grep -v '^#!' \
  | grep -v '# let' \
  | sed 's/^\/\/! \?//' \
  | sed 's/\[`\(.*\)`\](/[\1](https:\/\/docs.rs\/cantrip\/latest\/cantrip\/trait./' \
  | sed 's/::\(.*\)Y     |/.html#method.\1Y     |/' \
  | sed 's/::\(.*\)N     |/.html#tymethod.\1N     |/' \
  >>README.md.tmp
echo -e "\n" >>README.md.tmp
sed -n '/## Inspired by/,$ p' README.md >>README.md.tmp
mv -f README.md.tmp README.md

