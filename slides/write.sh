#!/bin/sh

fswatch -0 slides.adoc | xargs -0 -n1 -I{} make
