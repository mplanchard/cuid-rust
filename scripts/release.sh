#!/usr/bin/env bash

if [[ "$(git branch --show-current)" != "master" ]]; then
    echo -e "Must be on the master branch"
    exit 1
fi

# check for uncommitted changes
if ! git diff-index --quiet HEAD; then
    echo -e "Cannot have uncommitted changes"
    exit 1
fi

git fetch

if [[ -n "$(git log master..origin/master --oneline)" ]]; then
    echo -e "Branch is not up to date with origin"
    exit 1
fi
