# Platform reader contract

The shell, redaction, segmenting, and writer should stay shared. Only the
focused-window reader varies by desktop.

## Linux

Use the `atspi` crate to connect to the AT-SPI2 D-Bus and obtain the focused
application. Keep the connection alive for the capture thread. Walk only the
focused application's accessible subtree and collect text-bearing nodes.

Required checks:

- GNOME and KDE sessions with accessibility enabled.
- Focus changes between a browser, editor, terminal, and a password field.
- Password-role nodes are skipped at source.
- Wayland does not change the privacy boundary: no screenshots or compositor
  hooks are needed.
- A missing accessibility bus returns `None` and surfaces a setup message.

## Windows

Use the `uiautomation` crate on a dedicated COM-initialized thread. Call
`GetFocusedElement`, use the element's cached properties, and walk its
text-bearing descendants. Skip password controls at the UI Automation layer.

Required checks:

- Windows 10 and 11 with Edge, Chrome, VS Code, Notepad, and Windows Terminal.
- Focus changes and an already-destroyed element are retried safely.
- Password and secure-edit controls produce no text.
- UI Automation errors close the current segment instead of carrying it over.

## Shared acceptance test

For each platform, focus three ordinary windows for at least 15 seconds each,
then inspect the day file:

1. It contains app and window metadata, not screenshots.
2. Repeated text does not create an unbounded file.
3. Secrets are replaced before disk I/O.
4. Private browsing and password-manager windows produce no block.
5. Stopping Eyes flushes the open block and stops within one poll interval.
