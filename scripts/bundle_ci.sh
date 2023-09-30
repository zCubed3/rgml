#!/bin/bash

echo "Bundling CI files..."

echo "DLIB = ${CI_DLIB}"
echo "SLIB = ${CI_SLIB}"
echo "PFIX = ${CI_PFIX}"

mkdir ci_build
mkdir ci_build/c
mkdir ci_build/c/release
mkdir ci_build/c/debug

mv "target/debug/${CI_PFIX}rgml.${CI_DLIB}" ci_build/c/debug
mv "target/debug/${CI_PFIX}rgml.${CI_SLIB}" ci_build/c/debug
mv "target/release/${CI_PFIX}rgml.${CI_DLIB}" ci_build/c/release
mv "target/release/${CI_PFIX}rgml.${CI_SLIB}" ci_build/c/release
mv bindings/rgml.h ci_build/c

mkdir ci_build/dotnet
mkdir ci_build/dotnet/release
mkdir ci_build/dotnet/debug

mv bindings/dotnet/bin/Debug/net6.0/RGML.NET.dll ci_build/dotnet/debug
mv bindings/dotnet/bin/Release/net6.0/RGML.NET.dll ci_build/dotnet/release
cp "ci_build/c/debug/${CI_PFIX}rgml.${CI_DLIB}" ci_build/dotnet/debug
cp "ci_build/c/release/${CI_PFIX}rgml.${CI_DLIB}" ci_build/dotnet/release