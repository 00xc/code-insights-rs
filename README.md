# Code Insights types for Rust &emsp; ![Continuous integraton status][CI status] ![MIT licensed][license]

[CI status]: https://github.com/nossralf/code-insights-rs/workflows/CI/badge.svg
[license]: https://img.shields.io/github/license/nossralf/code-insights-rs

```toml
[dependencies]
code_insights = { git = "https://github.com/nossralf/code-insights-rs/" }
```

This crate contains Rust types that are useful when creating [Code
Insights][Atlassian code insights blog post] reports for Bitbucket Server.

It uses Serde for serialization.

`code_insights` does not contain any functionality for making the actual HTTP
requests to Bitbucket Server.

For more details about the REST resources used when interacting with Bitbucket
Server, see Atlassian's [API documentation][Atlassian Code Insights API documentation].

[Atlassian code insights blog post]: https://www.atlassian.com/blog/bitbucket/bitbucket-server-code-insights
[Atlassian Code Insights API documentation]: https://docs.atlassian.com/bitbucket-server/rest/6.10.1/bitbucket-code-insights-rest.html
