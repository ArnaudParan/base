#!/bin/bash

cd "$WDIR"
"$VENV/bin/pip" install -r requirements.txt
"$VENV/bin/gunicorn" --reload --bind="unix:$SOCK" pyBack.main:api
