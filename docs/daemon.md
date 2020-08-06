# Daemon
`spectapir` is a daemon that listens to the output of specific applications
and calculates statistics for the audio stream that can be used for
music visualization.

The profile
contains an estimation when the sound will likely be written to the real
soundcard. There is a second estimation based on the current card latency
reported by PA, that is, the time when the sound will likely be audible.
The rest of the profile is statistics on the sound played between this
profile and the last.