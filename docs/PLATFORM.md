# Platform reader contract

The shell, redaction, segmenting, and writer should stay shared. Only the
focused-window reader varies by desktop.

## Linux

The implemented reader uses the `atspi` crate to keep one AT-SPI2 D-Bus
connection for the capture thread. It locates the active frame, walks only
that frame's accessible subtree, caps nodes/text, and skips `PasswordText`
roles before the shared redaction boundary.

Required checks:

- GNOME and KDE sessions with accessibility enabled.
- Focus changes between a browser, editor, terminal, and a password field.
- Password-role nodes are skipped at source.
- Wayland does not change the privacy boundary: no screenshots or compositor
  hooks are needed.
- A missing accessibility bus returns `None` and surfaces a setup message.

## Windows

The implemented reader uses the `uiautomation` crate from the capture thread;
`UIAutomation::new` initializes COM before `GetFocusedElement`. It resolves
the window ancestor, walks the control tree, reads only `TextPattern`, caps
nodes/text, and skips any element whose `IsPassword` property is true.

Required checks:

- Windows 10 and 11 with Edge, Chrome, VS Code, Notepad, and Windows Terminal.
- Focus changes and an already-destroyed element are retried safely.
- Password and secure-edit controls produce no text.
- UI Automation errors close the current segment instead of carrying it over.

## Shared acceptance test

For each platform, focus three ordinary windows for at least 15 seconds each,
then inspect the day file:

1. It contains app and window metadata, not screenshots.
2. Repeated text during one Eyes session does not create an unbounded file.
3. Secrets are replaced before disk I/O.
4. Private browsing and password-manager windows produce no block.
5. Stopping Eyes flushes the open block and stops within one poll interval.
