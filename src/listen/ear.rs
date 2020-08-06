struct Ear {
    null_sink_module_id: u16,
    loopback_module_id: u16,
}

impl Ear {
    pub fn new() -> Self {
        // connect to pulseaudio daemon

        // check if plumbing is alredy present => crashed?
        // refuse startup

        // create null sink with name "tilti"
        // create monitor for the null sink
        // add loopback to re-route null sink to standard soundcard
        // add callback for pcm data on the null sink monitor

        // keep track of the module IDs both of
        unimplemented!()
    }
}

impl Drop for Ear {
    fn drop(&mut self) {
        let Ear {
            null_sink_module_id: _,
            loopback_module_id: _,
        } = self;
        // remove the callback
        // unload the module ids that we remember from startup
        // disconnect from pulseaudio daemon
        unimplemented!()
    }
}
