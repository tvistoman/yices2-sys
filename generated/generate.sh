#!/bin/sh

bindgen \
    --output bindings.rs \
    --whitelist-function 'yices(.*)$' \
    --generate functions \
    --no-doc-comments \
    --raw-line 'use libc::FILE;' \
    ../yices2/src/include/yices.h

bindgen \
    --output types.rs \
    --generate types \
    --whitelist-type '(.*)_[st]' \
    --no-doc-comments \
    --no-prepend-enum-name \
    ../yices2/src/include/yices.h

bindgen \
    --output vars.rs \
    --generate vars \
    --whitelist-var 'yices_version' \
    --no-doc-comments \
    --no-prepend-enum-name \
    ../yices2/src/include/yices.h

# bindgen \--whitelist-type '((.*)_[st])|(FILE)' \
#     --output yices_exit_codes.rs \
#     --generate types \
#     --blacklist-type '_(.*)$' \
#     --no-recursive-whitelist \
#     --no-doc-comments \
#     --no-prepend-enum-name \
#     ../yices2/src/include/yices_exit_codes.h