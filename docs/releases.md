# Releases

## Windows Collector

Releases are managed by Release Please. Commits merged to `main` should use
Conventional Commit prefixes such as `feat:` and `fix:`.

After changes land on `main`, the `Release Please` workflow opens or updates a
release PR. Merge that PR when you want to publish. Release Please will create
the GitHub Release, then the workflow builds the Windows collector on
`windows-2022` and attaches the zip.

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

## Windows Collector Integrity Checks

The CI and release workflows build the collector on explicit `windows-2022`
runners and run:

```powershell
.\scripts\verify-windows-collector.ps1 -ExePath .\target\release\collector-windows.exe -ReportDir .\dist
```

The verifier checks:

- the executable exists and can start far enough for `--version` to exit
- SHA-256 hash of the exe
- Authenticode status, currently warning when unsigned
- embedded application manifest
- Windows 10/11 compatibility declaration in the manifest
- `Microsoft.Windows.Common-Controls` v6 manifest dependency
- imported DLL dependencies through `dumpbin`
- whether app-local DLLs are sitting next to the exe and could shadow system DLLs
- Windows host version used for verification

The `--version` smoke test is intentionally simple: Windows resolves imported
DLLs before entering application code, so missing entry points such as
`GetWindowSubclass` should fail before a release asset is packaged.

When code signing is added, call the verifier with `-RequireSignature` after the
signing step.

The supported client floor is Windows 10 x64. GitHub-hosted Windows runners are
server SKUs, so CI gives a solid build/load/import signal on Windows NT 10.x but
does not replace a smoke test on a clean Windows 10 client VM before broad
distribution.
