# blue.toml

The blue.toml file is the main configuration file for blue. It is used to configure the workspace, the build system, and the project.

## [workspace]

The workspace section is used to configure the workspace.

### workspace.clean_targets

Targets that are cleaned when running [`blue clean`](/commands/workspace/clean).

```toml
[workspace]
clean_targets = ["target", "dist", ".cache"]
```
