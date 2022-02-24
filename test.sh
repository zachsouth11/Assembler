#!/usr/bin/env bash

# Build project
echo "Building PA1:"
cargo clean &>/dev/null
if ! cargo build --release &>/dev/null; then
    cargo check --release
    echo "FAILED TO BUILD!!! ABORTING!!!"
    exit 1
fi

ERROR=0

# Run test cases
echo
echo "Running tests:"
for i in tests/*.s; do
    cargo run --release "$i" &>/dev/null
    # echo "${i%.s}.expected"
    if diff "${i%.s}.o" "${i%.s}.expected" &>/dev/null; then
	echo "$i: passed"
    elif [ -f "${i%.s}.o" ]; then # Failed, output file exists
	ERROR=1
	if [ -s "${i%.s}.o" ]; then # Failed, output file is wrong
	    echo "$i: FAILED - Wrong Output"
	else # Failed, output file is empty
	    echo "$i: FAILED - Output Empty"
	fi
    else # Failed, no output file available
	ERROR=1
	echo "$i: FAILED - No Output"
    fi
done

exit $ERROR
