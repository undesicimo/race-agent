# Releases

## Windows Collector

Releases are managed by Release Please. Commits merged to `main` should use
Conventional Commit prefixes such as `feat:` and `fix:`.

After changes land on `main`, the `Release Please` workflow opens or updates a
release PR. Merge that PR when you want to publish. Release Please will create
the GitHub Release, then the workflow builds the Windows collector on
`windows-latest` and attaches the zip.

The release asset is named like:

```txt
race-agent-collector-windows-v0.1.0.zip
```

Repository setup required:

- Enable GitHub Actions.
- Allow GitHub Actions to create pull requests in Settings -> Actions -> General.
- Use Conventional Commit messages so Release Please can calculate versions.

Users download the zip, extract `collector-windows.exe`, double-click it, enter
the endpoint and token, then save/start the collector.
