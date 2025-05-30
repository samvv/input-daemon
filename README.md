InputDaemon
===========

InputDaemon is a service that runs as the `root` user and allows user-space
applications to control input devices such as a mouse or a keyboard.

**Why?**

This project came into being as a solution to [Wayland][1] restricting full
access to input devices due to security concerns. The design of InputDaemon
should still guarantee good security while enabling new applications that were
previously impossible.

[1]: https://wayland.freedesktop.org/

## Applications

 - [lan-mouse][2]

[2]: https://github.com/feschber/lan-mouse/pull/297
