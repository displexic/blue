# blue.toml

The blue.toml file is the main configuration file for blue. It is used to configure the workspace, the build system, and the project.

## Sections

- [build](#build)
- [features](#features)
- [update](#update)
- [workspace](#workspace)

## [build]

The build section is used to configure the build system.

::: info Unavailable
This section is not yet available.
:::

## [features]

Enable or disable features.

::: info Unavailable
This section is not yet available.
:::

## [update]

The update section is used to configure what is updated when running [`blue update`](/commands/workspace/update).

### update.dependencies

Supported dependencies:

- [`crates`](https://crates.io/)
- [`npm`](https://www.npmjs.com/)
- [`pip`](https://pypi.org/)

```toml
[update]
dependencies = ["cargo", "npm"]
```

### update.languages

Languages that are updated when running [`blue update`](/commands/workspace/update).

```toml
[update]
languages = ["rust", "nodejs"]
```

## [workspace]

The workspace section is used to configure the workspace.

### workspace.clean_targets

Targets that are cleaned when running [`blue clean`](/commands/workspace/clean).

```toml
[workspace]
clean_targets = ["target", "dist", "libs/*/dist"]
```

### workspace.projects

Projects included in the workspace.

```toml
[workspace]
projects = ["apps/my-app", "libs/*"]
```
