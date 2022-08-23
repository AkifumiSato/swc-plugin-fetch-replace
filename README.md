# swc-plugin-fetch-replacer

[![npm version](https://badge.fury.io/js/swc-plugin-fetch-replacer.svg)](https://badge.fury.io/js/swc-plugin-fetch-replacer)

SWC plugin for replaces global `fetch` with an arbitrary function.

## Install

```
$ pnpm add -D swc-plugin-fetch-replacer
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
          "swc-plugin-fetch-replacer",
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
