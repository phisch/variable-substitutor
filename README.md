# Substitutor

This is a little helper application that allows you to use color variables while writing a `zed-editor` theme. It might also be useful for other applications.

## Usage

Create a file called `variables.toml`, that contains your color variables:

```toml
[colors]
red = "#ff0000"
green = "#00ff00"
```

Now use those variables inside your theme template by prefixing their names with `$`. For example, create `custom.json.template` with the following content:

```json
{
  "$schema": "https://zed.dev/schema/themes/v0.1.0.json",
  "name": "Custom Theme",
  "author": "Philipp Schaffrath",
  "themes": [
    {
      "name": "Custom Theme Dark",
      "appearance": "dark",
      "style": {
        "editor.background": "$red",
        "panel.background": "$green"
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
