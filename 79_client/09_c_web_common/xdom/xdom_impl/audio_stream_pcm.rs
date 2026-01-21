use super::*;

pub struct Audio_Stream_Pcm {
    // _ctx: web_sys::AudioContext,
    // start: instant::Instant,
}

impl Audio_Stream_Pcm {
    pub fn pump_audio(self: &Rc<Audio_Stream_Pcm>) {
        /*
        loop {
            let t = instant::Instant::now();
            if t.duration_since(self.start).as_millis() > 50 {
            } else {
                /*

                */
            }
            // XdomA::sleep_millis(25);
        }

         */
        damn_it!("")
    }

    pub fn new() -> Rc<Audio_Stream_Pcm> {
        damn_it!("")
        /*
        let ctx = web_sys::AudioContext::new().unwrap();
        let gain = ctx.create_gain().unwrap();
        let start = instant::Instant::now();
        gain.gain().set_value(0.01);
        let _ = gain.connect_with_audio_node(&ctx.destination());

        let out = Rc::new(Audio_Stream_Pcm {
            _ctx: ctx,
            start: start,
        });

        out.pump_audio();

        out
        */
    }

    pub fn process(&self, _x: Vec<f32>) {}
}

impl Audio_Stream_Pcm {
    const _sample_rate: usize = 88200; // Hz
}

impl Drop for Audio_Stream_Pcm {
    fn drop(&mut self) {
        todo!()
    }
}
