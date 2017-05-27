# Language Tool provider for Patronus
This library provides [Language Tool](https://languagetool.org/) grammar checking library support to Patronus. It is based on [languagetol-rs](https://github.com/patronus-checker/languagetol-rs) library.

## Configuration
Since the provider requires LanguageTool API server, you may need to change the endpoint. You can add the following snippet to `$XDG_CONFIG_HOME/patronus/config.toml` (usually `$HOME/.config/patronus/config.toml`):

```toml
[providers.languagetool]
instance_url = "http://localhost:8081/"
```

There is also [public API server](http://wiki.languagetool.org/public-http-api) but it is limited and should not be used except for simple testing.
