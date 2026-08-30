# Security

Eyes is designed to keep captured context local. Please do not file public
issues for suspected privacy or data-exposure bugs.

Report them privately to the repository owner with reproduction steps,
affected platform, and the smallest relevant sample. Do not include real
passwords, API keys, account data, or private work content.

The first privacy boundary is the platform reader: it must read only the
focused accessible tree, cap traversal, skip secure fields, and return `None`
when the accessibility API is unavailable. The capture command writes only to
Eyes' app-data `daily/` directory. The shared redaction module is a second
line of defense, not permission to capture more data.
