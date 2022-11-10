#!/bin/bash

echo "Creating dummy files..."

mkdir target/
mkdir target/debug
mkdir target/release

touch target/debug/librgml.so
touch target/debug/librgml.a
touch target/release/librgml.so
touch target/release/librgml.a
touch bindings/rgml.h

mkdir -p bindings/dotnet/bin/Debug/net6.0
mkdir -p bindings/dotnet/bin/Release/net6.0

touch bindings/dotnet/bin/Debug/net6.0/RGML.NET.dll
touch bindings/dotnet/bin/Release/net6.0/RGML.NET.dll