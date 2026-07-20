# TODO

## Planned

- [ ] Recursive crawl: point at a NAS root and fix everything
- [ ] Date-prefix-aware idempotency: skip files already starting with
      `YYYY_MM_DD ` even if the embedded stamp doesn't match
- [ ] `-q` / `--quiet` flag: suppresses all non-error output (cron-friendly)
- [ ] `--skip-existing` flag: don't overwrite if dest path exists
- [ ] `--strict` flag: exit code 1 on any rename failure (script-safe)
- [ ] Shared MediaParser (Arc<Mutex<P>> or thread-local) for buffer reuse
      across parallel crawl
- [ ] More output formats (csv, json) for scripting
- [ ] Config file support for default flags
