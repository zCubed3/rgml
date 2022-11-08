#!/bin/bash

echo "Creating dummy files..."

mkdir target/
mkdir target/debug
mkdir target/release

touch target/debug/libprism_math.so
touch target/debug/libprism_math.a
touch target/release/libprism_math.so
touch target/release/libprism_math.a
touch bindings/prism_math.h

mkdir -p bindings/dotnet/bin/Debug/net6.0
mkdir -p bindings/dotnet/bin/Release/net6.0

touch bindings/dotnet/bin/Debug/net6.0/PrismMath.NET.dll
touch bindings/dotnet/bin/Release/net6.0/PrismMath.NET.dll