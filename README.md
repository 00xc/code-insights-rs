# Rust types for Bitbucket Code Insights &emsp; ![Continuous integraton status][CI status] ![MIT licensed][license]

[CI status]: https://github.com/nossralf/code-insights-rs/workflows/CI/badge.svg
[license]: https://img.shields.io/github/license/nossralf/code-insights-rs

```toml
[dependencies]
code_insights = { git = "https://github.com/nossralf/code-insights-rs/" }
```

This crate contains Rust types that are useful when creating [Code
Insights][Code Insights blog post] reports for Bitbucket Server.

It uses Serde for serialization.

`code_insights` does not contain any functionality for making the actual HTTP
requests to Bitbucket Server.

For more details about the REST resources used when interacting with Bitbucket
Server, see Atlassian's [how-to guide][Code Insights how-to guide],
[tutorial][Code Insights tutorial] and the Code Insights [API
documentation][Code Insights API documentation].

[Code Insights blog post]: https://www.atlassian.com/blog/bitbucket/bitbucket-server-code-insights
[Code Insights how-to guide]: https://developer.atlassian.com/server/bitbucket/how-tos/code-insights/
[Code Insights tutorial]: https://developer.atlassian.com/server/bitbucket/tutorials-and-examples/code-insights-tutorial/
[Code Insights API documentation]: https://docs.atlassian.com/bitbucket-server/rest/7.0.0/bitbucket-code-insights-rest.html
