# lol-item-analyzer cli

Provides utilities to access item info/analytics.

## Commands

### `loliac [--data-version <VERSION>] <SUBCOMMAND>`
---

| Option | Description |
|--------|-------------|
| `--data-version` | Set the version of the data to use, defaults to `latest` |

### `loliac -v`, `loliac --version`, `loliac version`
---

Print the CLI version.

### `loliac get <ITEM_NAME> [--strict]`
---

Get an item by item name. The `--strict` option will only look for items with the exact name,
otherwise the closest matching reasonable item will be returned.

### `loliac getid <ITEM_ID>`
---

Get an item by item id.


### `loliac search <TEXT>` 
---
`*Future Feature*`

Does a full text search for the given ] text.
