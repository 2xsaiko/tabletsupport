#!/bin/bash

DEVICES=('ELAN0732:00 04F3:2650 stylus' 'ELAN0732:00 04F3:2650 eraser')

case "$1" in
'normal')
    rotation='none'
    ;;
'left-up')
    rotation='ccw'
    ;;
'right-up')
    rotation='cw'
    ;;
'bottom-up')
    rotation='half'
    ;;
*)
    echo "Invalid rotation: $1"
    exit 1
esac

# something fucks with the Rotate setting after rotating the screen/display, so we need to wait until it has switched display modes
# 2 seconds seem to be enough for that to happen.
sleep 2

for dev in "${DEVICES[@]}"; do
    xsetwacom set "${dev}" Rotate "${rotation}"
done
