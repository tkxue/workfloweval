use super::*;
use crate::js_sys::Promise;

pub struct My_Bsn {
    // inner: web_sys::AudioBufferSourceNode,
}


impl Drop for My_Bsn {
    fn drop(&mut self) {
        damn_it!("")
        // self.inner.stop();
    }
}

pub struct Audio_Context {
    // raw_ctx: web_sys::AudioContext,
    // cur_node: Mutex<Option<My_Bsn>>,
}

impl Audio_Context {
    pub fn new() -> Audio_Context {
        Audio_Context {
            /*
            raw_ctx: web_sys::AudioContext::new().unwrap(),
            cur_node: Mutex::new(None),

             */
        }
    }

    pub fn current_time(&self) -> f64 {
        damn_it!("")
        // self.raw_ctx.current_time()
    }

    /*
    pub fn state(&self) -> web_sys::AudioContextState {
        self.raw_ctx.state()
    }

     */

    pub fn resume(&self) {
        damn_it!("")
        // let _ = self.raw_ctx.resume();
    }

    pub fn decode_ogg_2(&self, v: &[u8]) -> Result<Vec<f32>, Err_Frame> {
        damn_it!("")
        /*
        let b = std::io::Cursor::new(v);
        let mut stream = OggStreamReader::new(b).unwrap();

        let mut out = vec![];

        while let Some(packet) = stream.read_dec_packet().unwrap() {
            for (_, packets) in packet.iter().enumerate() {
                for x in packets.iter() {
                    out.push((*x as f32) / (i16::max_value() as f32));
                }
            }
        }

        Ok(out)
        */
    }

    pub fn play_data(&self, start_time: f64, data: &[f32], sample_rate: f32) {
        damn_it!("")
        /*
        wlog!("play_data, len: {:?}", data.len());
        let buffer = self.raw_ctx.create_buffer(1, data.len() as u32, sample_rate).unwrap();
        let _ = buffer.copy_to_channel(data, 0);

        let bsn = self.raw_ctx.create_buffer_source().unwrap();
        let _ = bsn.set_buffer(Some(&buffer));
        let _ = bsn.connect_with_audio_node(&self.raw_ctx.destination());

        // let _ = bsn.start_with_when(s);
        let _ = bsn.start().unwrap();
        // let _ = bsn.start_with_when(0.);

        *self.cur_node.lock().unwrap() = Some(My_Bsn { inner: bsn });
        */
    }

    pub async fn play_ogg(&self, data: &[u8]) {
        damn_it!("")
        /*
        wlog!("playing ogg");
        let t = instant::Instant::now();
        match self.decode_ogg_2(data) {
            Ok(v) => {
                let t2 = instant::Instant::now();

                wlog!("decode time: {:?}", (t2 - t).as_secs_f64());

                self.play_data(self.current_time(), v.as_slice(), 3000.);

                /*
                match js_sys::Float32Array::try_from(v)
                    .map_err(|_| wb::JsValue::from_str("Expected a Float32Array"))
                {
                    Ok(x) => {
                        let data: Vec<f32> = x.to_vec(); // unsafe { x.view() };

                        {
                            let sample_rate = 3000.0;
                            let duration = 2.0; // seconds
                            let frequency = 440.0; // Hz (A4 note)
                            let amplitude = 0.5; // 50% max volume

                            let sample_count = (duration * sample_rate) as usize;

                            // Generate sine wave PCM data
                            let mut pcm_data = vec![0.0f32; sample_count];
                            for t in 0..sample_count {
                                let time = t as f32 / sample_rate;
                                pcm_data[t] =
                                    amplitude * (2.0 * 3.1415926 * frequency * time).sin();
                            }

                            self.play_data(self.current_time(), pcm_data.as_slice(), 3000.);
                        }
                    }
                    Err(err) => {
                        wlog!("ogg decode to_f32 err: {:?}", err)
                    }
                };
                */
            }
            Err(x) => {
                wlog!("err: {:?}", x);
            }
        }
        */
    }
}
