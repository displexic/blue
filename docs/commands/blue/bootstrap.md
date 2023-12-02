# blue bootstrap

Installs Blue into the user's system.

::: warning Not meant for direct use
This command is meant to be used internally by Blue during installation. While it should not cause any problems, it is not recommended to use this command directly. However, in certain cases, it may be necessary to use this command directly to fix issues with Blue's installation.
:::

## Usage

```bash
blue bootstrap
```

## What it does

- Copies the Blue executable to the user:
  - Windows: `%USERPROFILE%\.blue\bin\blue.exe`
  - Linux: `~/.blue/bin/blue`
  - macOS: `/usr/local/bin/.blue/bin/blue`
- Adds the Blue executable to the user's PATH.
  - Windows: Using `setx` to add `%USERPROFILE%\.blue\bin` to the user's PATH.
  - Linux: `~/.bashrc`
  - macOS: `~/.bashrc_profile`
