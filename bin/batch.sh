#!/bin/sh -eu

MAIN=target/release/main 
GENERATOR=times
TILE=zigzag4

FROM=1
TO=256
SKIP=1
IMAGE_SIZE=3200

for i in $(seq $FROM $SKIP $TO)
do
  $MAIN --generator=$GENERATOR --tile=$TILE -i $IMAGE_SIZE --gp=0:1000000:$i -o "$GENERATOR/$TILE/$i.png"
  echo "$i done."
done
