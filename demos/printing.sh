#!/usr/bin/env bash

STEPS=${1:-10}

echo "steps: $STEPS"

for (( i=1; i<=$STEPS; i++ ))
do
	echo "🟢 TEST $i" ok
	sleep 0.9
done