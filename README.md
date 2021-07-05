# Data Standards Authority Workbench

## Who we are

We're the Data Standards Authority team, working to improve how the public sector manages data. We're establishing standards, writing guidance and building things to make it easier and more effective to share and use data across government.

You can find our web page on GOV.UK at https://www.gov.uk/government/groups/data-standards-authority, on UK Goverment Digital Slack in #data-standards-authority, or email us at data-standards-authority@digital.cabinet-office.gov.uk

## The workbench

The [workbench] is the place where the Data Standards Authority drafts its content. No content found there is official nor endorsed by the UK Government.

## Development

The workbench builder uses [Rust] and [Zola]. Once installed, use the following to compile the _hammer_ tool:

```sh
cd ./hammer
cargo build
```

### Architecture

The workbench architecture is a tranformation pipeline as follows:

- The canonical source is all the content in the `corpus` directory.
- `hammer` transforms the source into Zola content, stored in `workbench/content`.
- Finally `zola` builds the HTML to be deployed.


## Licence

Unless stated otherwise, the codebase is released under the [MIT licence].

The content is [© Crown copyright] and available under the terms of the [Open Government 3.0 licence].

[MIT Licence]: ./LICENCE
[© Crown copyright]: http://www.nationalarchives.gov.uk/information-management/re-using-public-sector-information/copyright-and-re-use/crown-copyright
[Open Government 3.0 licence]: https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3
[workbench]: https://alphagov.github.io/data-standards-authority/
[Zola]: https://www.getzola.org/
[Rust]: https://www.rust-lang.org/
