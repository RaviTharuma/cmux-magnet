# Plugin design

- `cmux-plugin.toml`: sidebar kind (moshi/herdr pattern); `run` host binary + `cmux-magnet apply` for layouts.
- Swift SPM library embeds in an ExtensionKit sidebar extension (`com.cmuxterm.app.cmux.sidebar`).
- Magnet drag zones require host APIs beyond public CmuxExtensionKit — upstream #12230.
