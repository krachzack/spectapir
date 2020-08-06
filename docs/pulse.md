# PulseAudio
This is what `spectapir` uses on Linux to:
* create an artificial soundcard to write sound into that should be analyzed,
* play that sound on the default soundcard using a loopback setup.

## Configuration
Interaction of `spectapir` with the daemon is equivalent to the following 
`default.pa`, which can be put e.g. into `~/.config/pulse`:
```
.include /etc/pulse/default.pa
# Add a null sink (with a monitor source) for applications that should contribute to visualization
load-module module-null-sink sink_name=VisualOut format=s16be channels=2 rate=44100 sink_properties="device.description='Visuals (Output)'"
# Loopback with one second of delay to the default output
load-module module-loopback source=VisualOut.monitor source_dont_move=yes latency_msec=1000 
```

If `VisualOut` is not found at startup, the daemon will connect to the
pulse audio daemon and create it automatically, removing it again at
daemon shutdown, by remembering the module IDs returned after loading
the modules and issuing `unload-module` invocations:
```
unload-module 42
unload-module 43
```

## C API
pulseaudio has several APIs, one is C-based and async:
https://freedesktop.org/software/pulseaudio/doxygen/

Examples using async:
https://www.freedesktop.org/wiki/Software/PulseAudio/Documentation/Developer/Clients/Samples/AsyncDeviceList/
https://www.freedesktop.org/wiki/Software/PulseAudio/Documentation/Developer/Clients/Samples/AsyncPlayback/

## Rust API
There seem to be nice rust bindings:
https://docs.rs/libpulse-binding/2.16.0/libpulse_binding/