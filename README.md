# del-ghcr

You can delete all untagged images of GitHub Container Registry (ghcr.io) by using this crate.

## Usage

`docker run ghcr.io/lucky3028/del-ghcr:latest --token <token> --container <container> --dry-run`

|Options|Description|Shorthand|Is Required|
|---|---|---|---|
|`--token [token]`|The token of GitHub Container Registry. You can pass the `GHCR_TOKEN` environmental variable, instead of this option.|`-t`|✔|
|`--container [container_name]`|Container name. For example, `del-ghcr`. The name doesn't start from `ghcr.io`.|`-c`|✔|
|`--dry-run`|Shows the list of all untagged images.|`-d`|-|

## License

[MIT License](./LICENSE)
