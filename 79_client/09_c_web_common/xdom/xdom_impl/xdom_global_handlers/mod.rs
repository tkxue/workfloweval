use super::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use xdom_api2::*;

mod conv_keyboard;
mod winit_keyboard;
pub use conv_keyboard::*;
pub use winit_keyboard::*;

pub struct Xdom_Global_Handlers_Inner {
    on_message: RefCell<Option<Rc<Closure<dyn Fn(my_ffi::message_event::MessageEvent)>>>>,
    /*
    on_resize: RefCell<Option<Rc<Closure<dyn Fn(web_sys::UiEvent)>>>>,
    on_keyboard: RefCell<Option<Rc<Closure<dyn Fn(web_sys::KeyboardEvent)>>>>,
    nf: RefCell<Option<Rc<Closure<dyn Fn(web_sys::MouseEvent)>>>>,
    nf2: RefCell<Option<Rc<Closure<dyn Fn(web_sys::PointerEvent)>>>>,
    nf3: RefCell<Option<Rc<Closure<dyn Fn(web_sys::WheelEvent)>>>>,
    on_pointer_lock_change: RefCell<Option<Rc<Closure<dyn Fn(web_sys::Event)>>>>,

     */
}

/*
impl Xdom_Global_Handlers_Inner {
    pub fn set_on_message(&self, cb: Option<Rc<dyn Fn(XdomA_Message_Event)>>) {
        /*
        let window: web_sys::Window = web_sys::window().un(err!("can't get window"));
        match cb {
            None => {
                self.on_message.replace(None);
                window.set_onmessage(None);
            }
            Some(f) => {
                let nf = Rc::new(Dom_Util::wrap(move |x: web_sys::MessageEvent| {
                    (f.as_ref())(XdomA_Message_Event { inner: x.into() })
                }));
                self.on_message.replace(Some(nf.clone()));
                window.set_onmessage(Some(nf.as_ref().as_ref().unchecked_ref()));
            }
        }

         */


        let window = &my_ffi::window::WINDOW;

        match cb {
            None => {
                self.on_message.replace(None);
                window.set_onmessage(None);
            }
            Some(f) => {
                let closure = Closure::<dyn Fn(my_ffi::message_event::MessageEvent)>::new(move |x: my_ffi::message_event::MessageEvent| {
                    (f.as_ref())(XdomA_Message_Event { inner: x.into() })
                });

                let js_func = closure.as_ref().unchecked_ref::<js_sys::Function>();

                window.set_onmessage(Some(js_func));

                self.on_message.replace(Some(Rc::new(closure)));
            }
        }





    }

    /*
    pub fn set_on_keyboard(&self, cb: Option<Rc<dyn Fn(Key_Event)>>) {
        let window: web_sys::Window = web_sys::window().un(err!("can't get window"));
        match cb {
            None => {
                self.on_keyboard.replace(None);
                window.set_onkeydown(None);
                window.set_onkeyup(None);
            }
            Some(f) => {
                let nf = Rc::new(Dom_Util::wrap(move |x: web_sys::KeyboardEvent| {
                    x.prevent_default();
                    x.stop_propagation();
                    let evt = Xdom_Global_Handlers_Inner::conv_keyboard(&x);

                    match evt.key_raw {
                        Key_Raw::Special(Key_Special::Escape) => return,
                        Key_Raw::Special(Key_Special::LControl) => return,
                        Key_Raw::Special(Key_Special::RControl) => return,
                        Key_Raw::Special(Key_Special::LAlt) => return,
                        Key_Raw::Special(Key_Special::RAlt) => return,
                        Key_Raw::Special(Key_Special::LShift) => return,
                        Key_Raw::Special(Key_Special::RShift) => return,
                        _ => {}
                    }

                    if evt.key_raw == Key_Raw::Normal(Key_Normal::Semicolon)
                        && evt.modifiers.contains(Xgpu_Modifiers_State::CTRL)
                    {
                        return;
                    }

                    match evt.state {
                        Key_State::Pressed => XdomA::request_fullscreen(),
                        Key_State::Released => {}
                    }
                    (f.as_ref())(evt)
                }));
                self.on_keyboard.replace(Some(nf.clone()));
                window.set_onkeydown(Some(nf.as_ref().as_ref().unchecked_ref()));
                window.set_onkeyup(Some(nf.as_ref().as_ref().unchecked_ref()));
            }
        }
    }

    pub fn set_on_mouse(&self, cb: Option<Rc<dyn Fn(Mouse_Event_Raw)>>) {
        let window: web_sys::Window = web_sys::window().un(err!("can't get window"));
        match cb {
            None => {
                self.nf.replace(None);
                self.nf2.replace(None);
                window.set_onmousemove(None);
                window.set_onmouseup(None);
                window.set_onmousedown(None);
                window.set_oncontextmenu(None);
                window.set_onwheel(None);
            }
            Some(f) => {
                let nf = {
                    let f = f.clone();
                    Rc::new(Dom_Util::wrap(move |x: web_sys::MouseEvent| {
                        x.prevent_default();
                        x.stop_propagation();
                        let evt = Xdom_Global_Handlers_Inner::conv_mouse(&x);
                        match evt.data.state {
                            Mouse_Event_State::Mouse_Move => {}
                            Mouse_Event_State::Mouse_Up => {}
                            Mouse_Event_State::Mouse_Down => XdomA::request_fullscreen(),
                            Mouse_Event_State::Context_Menu => {}
                            Mouse_Event_State::Wheel => {}
                        }
                        (f.as_ref())(evt)
                    }))
                };
                let nf2 = {
                    let f = f.clone();
                    Rc::new(Dom_Util::wrap(move |x: web_sys::PointerEvent| {
                        // x.prevent_default();
                        // x.stop_propagation();
                        (f.as_ref())(Xdom_Global_Handlers_Inner::conv_pointer(&x))
                    }))
                };
                let nf3 = {
                    let f = f.clone();
                    Rc::new(Dom_Util::wrap(move |x: web_sys::WheelEvent| {
                        // x.prevent_default();
                        // x.stop_propagation();
                        (f.as_ref())(Xdom_Global_Handlers_Inner::conv_wheel(&x))
                    }))
                };
                self.nf.replace(Some(nf.clone()));
                self.nf2.replace(Some(nf2.clone()));
                self.nf3.replace(Some(nf3.clone()));
                window.set_onmousemove(Some(nf.as_ref().as_ref().unchecked_ref()));
                window.set_onmouseup(Some(nf.as_ref().as_ref().unchecked_ref()));
                window.set_onmousedown(Some(nf.as_ref().as_ref().unchecked_ref()));
                window.set_oncontextmenu(Some(nf2.as_ref().as_ref().unchecked_ref()));
                window.set_onwheel(Some(nf3.as_ref().as_ref().unchecked_ref()));
            }
        }
    }

    pub fn set_on_resize(&self, cb: Option<Rc<dyn Fn(XdomA_UI_Resize_Event)>>) {
        let window: web_sys::Window = web_sys::window().un(err!("can't get window"));
        match cb {
            None => {
                self.on_resize.replace(None);
                window.set_onresize(None);
            }
            Some(f) => {
                let nf = Rc::new(Dom_Util::wrap(move |x: web_sys::UiEvent| {
                    (f.as_ref())(XdomA_UI_Resize_Event { inner: x.into() })
                }));
                self.on_resize.replace(Some(nf.clone()));
                window.set_onresize(Some(nf.as_ref().as_ref().unchecked_ref()));
            }
        }
    }

    pub fn set_on_pointer_lock_change(&self, cb: Option<Rc<dyn Fn(XdomA_Pointer_Event)>>) {
        let window: web_sys::Window = web_sys::window().un(err!("can't get window"));
        let doc = window.document().un(err!(""));
        match cb {
            None => {
                self.on_pointer_lock_change.replace(None);
                doc.set_onpointerlockchange(None)
            }
            Some(cb) => {
                let nf = Rc::new(Dom_Util::wrap(move |x: web_sys::Event| {
                    (cb.as_ref())(XdomA_Pointer_Event { inner: x.into() })
                }));
                self.on_pointer_lock_change.replace(Some(nf.clone()));
                doc.set_onpointerlockchange(Some(nf.as_ref().as_ref().unchecked_ref()));
            }
        }
    }
    */
}
*/

impl Xdom_Global_Handlers_Inner {
    pub fn new() -> Xdom_Global_Handlers_Inner {
        Xdom_Global_Handlers_Inner {
            on_message: RefCell::new(None),
            /*
            on_resize: RefCell::new(None),
            on_keyboard: RefCell::new(None),
            nf: RefCell::new(None),
            nf2: RefCell::new(None),
            nf3: RefCell::new(None),
            on_pointer_lock_change: RefCell::new(None),

             */
        }
    }
}

impl Drop for Xdom_Global_Handlers_Inner {
    fn drop(&mut self) {
        /*
        self.set_on_message(None);
        self.set_on_resize(None);

         */
        damn_it!("")
    }
}

#[derive(Clone)]
pub struct Xdom_Global_Handlers {
    pub inner: Rc<Xdom_Global_Handlers_Inner>,
}

impl Xdom_Global_Handlers {
    pub fn new() -> Xdom_Global_Handlers {
        Xdom_Global_Handlers {
            inner: Rc::new(Xdom_Global_Handlers_Inner::new()),
        }
    }
}
