# License

PRECC offers two tiers: Community (free) and Pro.

## Community Tier (Free)

The Community tier includes:

- All built-in skills (wrong-directory correction, jj translation, etc.)
- Hook pipeline with full Pillar 1 and Pillar 4 support
- Basic `precc savings` summary
- Session mining with `precc ingest`
- Unlimited local usage

## Pro Tier

Pro unlocks additional features:

- **Detailed savings breakdown** -- `precc savings --all` with per-command analysis
- **GIF recording** -- `precc gif` for creating animated terminal GIFs
- **IP geofence compliance** -- For regulated environments
- **Email reports** -- `precc mail report` to send analytics
- **GitHub Actions analysis** -- `precc gha` for failed workflow debugging
- **Context compression** -- `precc compress` for CLAUDE.md optimization
- **Priority support**

## Activating a License

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Checking License Status

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## GitHub Sponsors Activation

If you sponsor PRECC through GitHub Sponsors, your license is activated automatically via your GitHub email. No key required -- just ensure your sponsor email matches:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Device Fingerprint

Each license is tied to a device fingerprint. View yours with:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

If you need to transfer your license to a new machine, deactivate first:

```bash
precc license deactivate
```

Then activate on the new machine.

## License Expired?

When a Pro license expires, PRECC reverts to Community tier. All built-in skills and core functionality continue to work. Only Pro-specific features become unavailable. See the [FAQ](faq.md) for more details.
