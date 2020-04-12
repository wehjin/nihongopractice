#!/bin/sh
rm -rf docs/*.js docs/*.html docs/*.wasm
npm run build -- --mode=production
