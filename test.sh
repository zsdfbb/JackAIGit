#!/bin/bash

set -e

echo "Run AIGit test..."

command -v aigit >/dev/null 2>&1 || {
  echo >&2 "aigit is not installed. Aborting."
  exit 1
}

# =============================================================
# prepare test
echo "# test" > ./install.sh

# =============================================================
# do test

aigit show -e 5965fdf805c4c81bd709980ab2ba2b12f04fbf19

aigit diff -e

git add ./
aigit diff -e -s

aigit commit -e -d

# =============================================================
# restore
git reset --soft HEAD^
git restore --staged ./install.sh
git checkout -- ./install.sh
