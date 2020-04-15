#!/bin/sh
rm -rf docs/*.js docs/*.html docs/*.wasm docs/clips/
npm run build -- --mode=production
cp -rf clips docs/
