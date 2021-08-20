#!/bin/bash

cd "$WDIR"
gunicorn --bind="${GHOST}" pyBack.main:api

