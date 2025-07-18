#!/bin/bash

# check git install
if ! command -v git &> /dev/null
then
    echo "Git is not installed. Please install Git and try again."
    exit 1
fi