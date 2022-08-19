#!/bin/bash  

cd ..
cd src

echo "Checkout to branch dev"
git checkout dev

echo "Add Files"
git add -A

echo "Enter Commit Name (Default is OpenLaunch-Commit):"
read commit_name

if [ $commit_name == "\n" ];
then
commit_name="OpenLaunch-Commit"
fi

echo "Commit Files"
git commit -m "$commit_name"

echo "Commit Finished"
echo "Run 002_Push-to.sh to push to remote"
echo "Do you want to push now ? [Y/N] :"
read push_flag

if [ $push_flag == "Y" ];
then
cd ..
cd script
bash 002_Push-to.sh
fi

if [ $push_flag == "N" ];
then
echo "Shell Execution Finished."
fi