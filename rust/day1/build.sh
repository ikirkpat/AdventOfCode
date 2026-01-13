#!/bin/bash
set -e

rm -rf debug
rustc main.rs --out-dir debug
