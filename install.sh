#!/bin/bash

set -e

# check git install
if ! command -v git &> /dev/null
then
    echo "Git is not installed. Please install Git and try again."
    exit 1
else
    echo "Git is installed."
fi


# check git install
if ! command -v aigit &> /dev/null
then
    echo "AIGit is not installed. Install AIGit ..."
else
    echo "AIGit is installed. Reinstall AIGit ..."
fi

# install aigit
pushd aigit
cargo build --release
sudo cp target/release/aigit /usr/local/bin
popd

mkdir -p ${HOME}/.config/aigit
cp ./aigit.toml ${HOME}/.config/aigit/config.toml

echo "Install AIGit successfully."
