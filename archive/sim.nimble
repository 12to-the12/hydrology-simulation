# Package

version       = "0.1.0"
author        = "Logan Hillyer"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
bin           = @["sim"]


# Dependencies

requires "nim >= 2.2.6"
requires "nimpy"
requires "pixie"
requires "noisy"

task release_clang, "Build a production release (macOS)":
  --verbose
  --forceBuild:on
  --cc:clang
  --define:release
  --deepcopy:on
  --cpu:arm64
  --passC:"-flto -target arm64-apple-macos11" 
  --passL:"-flto -target arm64-apple-macos11"
  --hints:off
  --outdir:"."
  setCommand "c", "./src/sim.nim"