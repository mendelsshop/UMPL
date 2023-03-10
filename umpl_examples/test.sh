#!/bin/bash

# This script is used to test the umpl_examples package.
# it runs the examples and sees that they do not fail.

# it skips the examples that ask for user input.

# get list of examples
examples=`ls umpl_examples/*.umpl`
# skip example.umpl and loop.umpl and simple.umpl
examples=`echo $examples | sed 's/umpl_examples\/example.umpl//g'`
examples=`echo $examples | sed 's/umpl_examples\/loop.umpl//g'`
examples=`echo $examples | sed 's/umpl_examples\/simple.umpl//g'`

echo "Running examples: $examples"

# build umpl

cargo build --release

# run examples
for example in $examples
do
    echo "Running $example"
    cargo run --release -- $example > /dev/null
    if [ $? -ne 0 ]; then
        cargo run --release -- $example
        echo "Error running $example"
        # exit 1
    fi
done
