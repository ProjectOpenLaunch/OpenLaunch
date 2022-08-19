#!/bin/bash  

cd ..

echo "Start Build Project."

echo "Confirm? [Y/N]"
read build_flag

if [ $build_flag == "Y" ];
then
rustup update
cargo version
cargo build
fi

if [ $build_flag == "N" ];
then
echo "Build Canceled."
fi