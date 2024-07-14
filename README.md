# Substitutor

This is a little helper application that allows you to use color variables while writing a `zed-editor` theme. It might also be useful for other applications.

## Usage

Create a file called `variables.toml`, that contains your color variables:

```toml
alert = "#ffcc00"

[accent]
blue = "#497EE9"

[muted]
purple = "#382B72"

[surface]
primary = "#0A0A0A"
secondary = "#141414"

```

Now use those variables inside your theme template by prefixing their names with `$`. For example, create `custom.json.template` with the following content:

```json
{
  "$schema": "https://zed.dev/schema/themes/v0.1.0.json",
  "name": "Custom",
  "author": "Philipp Schaffrath",
  "themes": [
    {
      "name": "Custom Dark",
      "appearance": "dark",
      "style": {
        "title_bar.background": "$accent.blue",
        "editor.background": "$surface.primary",
        "panel.background": "$surface.secondary",
        "status_bar.background": "$muted.purple",
        "error.background": "$alert"
      }
    }
  ]
}


```
And finally, run:

```sh
substitutor custom.json.template
```

This will look for the `variables.toml` file in the same directory as `custom.json.template`, and output to `~/.config/zed/themes/custom.json`. You can overwrite those paths using `--variables` and `--output`. For more information, use `--help`.

> [!TIP]
> Use `--watch` to automatically update your theme whenever you changed your template or variables.
