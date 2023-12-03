# blue update

Updates dependencies, languages, package managers, and other tools.

Alias: `up`

## Environment

## Options

### `--all`

Updates all dependencies, languages, package managers, and other tools.

### `--dependencies`

Updates dependencies.

### `--languages`

Updates languages.

### `--package-managers`

Updates package managers.

### `--self`

Updates Blue.

### `--tools`

Updates tools.

### `--workspace`

Updates the workspace.

## blue.toml settings

```toml
[update]
dependencies = ["cargo", "npm"]
languages = ["rust", "nodejs"]
package_managers = ["pnpm"]
self = true
tools = false
```
