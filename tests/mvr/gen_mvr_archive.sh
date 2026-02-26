#!/bin/bash
(
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  cd "$SCRIPT_DIR/sample_show" && zip -r -Z deflate sample_show.mvr ./*
)
