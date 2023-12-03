# blue clean

Cleans up temporary files and directories.

::: danger Danger
This command will delete files and directories without confirmation.
:::

## Usage

```bash
blue clean [target...]
```

## blue.toml settings

```toml
[workspace]
clean_targets = ["target", "dist", ".cache"]
```
