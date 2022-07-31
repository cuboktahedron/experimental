#!/bin/sh -eu

MAIN=target/release/main 
GENERATOR=primes
TILE=spiral6

FROM=1
TO=1
SKIP=1
IMAGE_SIZE=3200

for i in $(seq $FROM $SKIP $TO)
do
  $MAIN --generator=$GENERATOR --tile=$TILE -i $IMAGE_SIZE --gp=0:100000:$i -o "$GENERATOR/$TILE/$i.png"
  echo "$i done."
done
