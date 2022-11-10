#!/bin/bash

echo "Creating dummy files..."

mkdir target/
mkdir target/debug
mkdir target/release

touch target/debug/rgml.so
touch target/debug/rgml.a
touch target/release/rgml.so
touch target/release/rgml.a
touch bindings/rgml.h

mkdir -p bindings/dotnet/bin/Debug/net6.0
mkdir -p bindings/dotnet/bin/Release/net6.0

touch bindings/dotnet/bin/Debug/net6.0/RGML.NET.dll
touch bindings/dotnet/bin/Release/net6.0/RGML.NET.dll