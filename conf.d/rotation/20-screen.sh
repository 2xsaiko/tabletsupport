#!/bin/bash

SCREEN='eDP1'

case "$1" in
'normal')
    rotation='normal'
    ;;
'left-up')
    rotation='left'
    ;;
'right-up')
    rotation='right'
    ;;
'bottom-up')
    rotation='inverted'
    ;;
*)
    echo "Invalid rotation: $1"
    exit 1
esac

xrandr --output "${SCREEN}" --rotate "${rotation}"
