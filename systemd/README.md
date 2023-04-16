# Systemd Service

When deploying to a remote device running Linux with system, these services can be useful to automatically reload the Pixelboard program
when copying a new binary over.

To setup, in this directory run:

```
systemd $ scp pixelboard* <remote_user>@<remote_hostname>:/lib/systemd/system/
```

Start a shell session on the remote device and enable the pixelboard and pixelboard-watcher service

```
systemctl enable pixelboard && systemctl start pixelboard
systemctl enable pixelboard-watcher && systemctl start pixelboard-watcher
systemctl enable pixelboard-watcher.path && systemctl start pixelboard-watcher.path
```

Whenever you copy the binary over to `/app/pixelboard/pixelboard` the app will restart with the new binary.
Whenever you restart the device, Pixelboard will automatically start.
