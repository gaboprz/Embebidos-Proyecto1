#!/bin/sh

VIDEO="${1:-/usr/share/videos/video.mp4}"

gst-launch-1.0 filesrc location="$VIDEO" ! decodebin ! videoconvert ! fbdevsink
