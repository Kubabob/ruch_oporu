#!/usr/bin/env bash
set -euo pipefail

# Build the project
trunk build --release --public-url "/$REPO_NAME/"

# Create .nojekyll file to disable Jekyll processing
touch dist/.nojekyll