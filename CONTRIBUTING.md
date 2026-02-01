Making contributions envolves some things.

# Versioning

To automatically have new versions and releases, it's essential to follow a commit standard: [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

This project uses the [Git Cliff](https://github.com/orhun/git-cliff) crate along with the `mise release` command to perform **automatic version bumps**, create tags and releases on GitHub, and update the `CHANGELOG.md`.

# Recommended Commit Structure

\<type\>(\<optional scope\>): \<short message\>

- **type**: `fix`, `feat`, `perf`, `security`, etc.
- **scope**: affected module or component (optional but recommended).
- **message**: brief description of what changed.

> Examples:
>
> ```bash
> git commit -m "fix(auth): fix login bug"
> git commit -m "feat(export): add PDF export"
> git commit -m "perf(db): optimize report query"
> git commit -m "security(auth): fix CSRF vulnerability"
> git commit -m "feat(api): new login route (BREAKING CHANGE)"
> ```

# Commit Types and Version Impact

| Commit Prefix     | Version Bump    | Meaning                                                  |
| ----------------- | --------------- | -------------------------------------------------------- |
| `fix`             | Patch (`*.*.X`) | Bug fix without changing existing functionality.         |
| `perf`            | Patch (`*.*.X`) | Performance improvement without breaking compatibility.  |
| `security`        | Patch (`*.*.X`) | Fix for security vulnerabilities.                        |
| `feat`            | Minor (`*.X.*`) | New feature or compatible improvement.                   |
| `BREAKING CHANGE` | Major (`X.*.*`) | Change that breaks compatibility with previous versions. |

> ⚠️ Commits with prefixes like `chore`, `build`, `ci`, `docs`, `style`, `test`, or `refactor` **do not generate a version bump**, but still appear in the changelog.
