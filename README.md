# swc-plugin-fetch-replace

[![npm version](https://badge.fury.io/js/swc-plugin-fetch-replace.svg)](https://badge.fury.io/js/swc-plugin-fetch-replace)

SWC plugin for replaces global `fetch` with an arbitrary function.

## Install

```
$ pnpm add -D swc-plugin-fetch-replace
```

## Configuration

Function name with `replace_fetch`.

_.swcrc_
```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-fetch-replace",
          {
            "replaceName": "replace_fetch"
          }
        ]
      ]
    }
  }
}
```

## todo

- [x] add examples
- [ ] add jest examples
- [ ] summarize in an article
