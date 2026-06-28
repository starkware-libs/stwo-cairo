#!/bin/bash
docker build --quiet . --file Dockerfile_cairo_compile -t cairo_compile > /dev/null 2> /dev/null
docker run --quiet -v .:/inputs cairo_compile $@
