# del-ghcr

You can delete all untagged images of GitHub Container Registry (ghcr.io) by using this crate.

## Usage

`/del-ghcr --token <token> --container <container> --force --dry-run`

|Options|Description|Shorthand|Is Required|
|---|---|---|---|
|`--token [token]`|The token of GitHub Container Registry. You can pass the `GHCR_TOKEN` environmental variable, instead of this option.|`-t`|✔|
|`--container [container_name]`|Container name. For example, `del-ghcr`. The name doesn't start from `ghcr.io`.|`-c`|✔|
|`--force`|Delete images without a confirmation message.|`-f`|-|
|`--dry-run`|Shows the list of all untagged images.|`-d`|-|

## Release

You can download the executable file from
* [here](https://github.com/Lucky3028/del-ghcr/releases) (GitHub Release)
* [here](./download.sh) (bash script)

## License

[MIT License](./LICENSE)
