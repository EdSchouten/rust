#!/bin/bash
# Copyright 2018 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -eux

target="$1"

# Based on the steps described at https://nuxi.nl/cloudabi/debian/

# Set up a Clang-based cross compiler toolchain.
for tool in ar nm objdump ranlib size; do
  ln -s ../lib/llvm-5.0/bin/llvm-${tool} /usr/bin/${target}-${tool}
done
ln -s ../lib/llvm-5.0/bin/clang /usr/bin/${target}-cc
ln -s ../lib/llvm-5.0/bin/clang /usr/bin/${target}-c++
ln -s ../lib/llvm-5.0/bin/lld /usr/bin/${target}-ld
ln -s ../../${target} /usr/lib/llvm-5.0/${target}

# TODO(ed): Extra symlinks for -clang and -clang++ to work around
# https://github.com/alexcrichton/cc-rs/pull/278
ln -s ../lib/llvm-5.0/bin/clang /usr/bin/${target}-clang
ln -s ../lib/llvm-5.0/bin/clang /usr/bin/${target}-clang++

# Install the C++ runtime libraries from CloudABI Ports.
echo deb https://nuxi.nl/distfiles/cloudabi-ports/debian/ cloudabi cloudabi > /etc/apt/sources.list.d/cloudabi.list
curl 'https://pgp.mit.edu/pks/lookup?op=get&search=0x0DA51B8531344B15' | apt-key add -
apt-get update
apt-get install -y $(echo ${target} | sed -e s/_/-/g)-cxx-runtime
