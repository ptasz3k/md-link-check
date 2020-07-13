# md-link-check
Check for broken links in md documents

Done quickly out of frustration caused by slowness of node.js markdown link checker.

Usage:

```
md-link-check [--local-only] [--ascii-only] [--print-successes] [<directory>]
```

--local-only: check local links only, by default program checks also urls (http:// and https://),

--ascii-only: allow only ASCII characters in links,

--print-successes: print correct links, by default program prints only broken ones,

\<directory\>: directory in which recursive search should start, program will check all files with `.md` extension. Set to `.` (current dir) by default.

Exit status: 1 if at least one broken link was found, 0 if none.

TODO:

* parallel file checking.

