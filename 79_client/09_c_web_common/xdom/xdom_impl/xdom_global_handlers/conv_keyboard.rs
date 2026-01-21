use super::*;


impl Xdom_Global_Handlers_Inner {
    /*
    pub fn conv_keyboard(e: &web_sys::KeyboardEvent) -> Key_Event {
        Key_Event {
            state: match e.type_().as_ref() {
                "keydown" => Key_State::Pressed,
                "keyup" => Key_State::Released,
                _ => damn_it!(""),
            },
            key_raw: Winit_Conv_Keyboard::from_key_code_attribute_value(e.code().as_ref()),
            modifiers: Xgpu_Modifiers_State::new(
                e.shift_key(),
                e.ctrl_key(),
                e.alt_key(),
                e.meta_key(),
            ),
        }
    }

    pub fn conv_mouse(e: &web_sys::MouseEvent) -> Mouse_Event_Raw {
        Mouse_Event_Raw {
            top_left_zero_client_coord: Pt2_i32::new_x_y(e.client_x(), e.client_y()),
            data: Mouse_Event_Data {
                state: match e.type_().as_ref() {
                    "mousedown" => Mouse_Event_State::Mouse_Down,
                    "mouseup" => Mouse_Event_State::Mouse_Up,
                    "mousemove" => Mouse_Event_State::Mouse_Move,
                    s => damn_it!("unrecognized mouse event: {}", s),
                },
                // movement_x: e.movement_x(),
                // movement_y: e.movement_y(),
                ctrl: e.ctrl_key(),
                shift: e.shift_key(),
                alt: e.alt_key(),
                meta: e.meta_key(),
                button: e.button(),
                buttons: Mouse_Buttons { inner: e.buttons() },
                delta_x: 0.0,
                delta_y: 0.0,
                delta_z: 0.0,
            },
        }
    }

    pub fn conv_pointer(e: &web_sys::PointerEvent) -> Mouse_Event_Raw {
        Mouse_Event_Raw {
            top_left_zero_client_coord: Pt2_i32::new_x_y(e.client_x(), e.client_y()),
            data: Mouse_Event_Data {
                state: match e.type_().as_ref() {
                    "mousedown" => Mouse_Event_State::Mouse_Down,
                    "mouseup" => Mouse_Event_State::Mouse_Up,
                    "mousemove" => Mouse_Event_State::Mouse_Move,
                    "contextmenu" => Mouse_Event_State::Context_Menu,
                    s => damn_it!("unrecognized mouse event: {}", s),
                },
                // movement_x: e.movement_x(),
                // movement_y: e.movement_y(),
                ctrl: e.ctrl_key(),
                shift: e.shift_key(),
                alt: e.alt_key(),
                meta: e.meta_key(),
                button: e.button(),
                buttons: Mouse_Buttons { inner: e.buttons() },
                delta_x: 0.0,
                delta_y: 0.0,
                delta_z: 0.0,
            },
        }
    }

    pub fn conv_wheel(e: &web_sys::WheelEvent) -> Mouse_Event_Raw {
        Mouse_Event_Raw {
            top_left_zero_client_coord: Pt2_i32::new_x_y(e.client_x(), e.client_y()),
            data: Mouse_Event_Data {
                state: match e.type_().as_ref() {
                    "mousedown" => Mouse_Event_State::Mouse_Down,
                    "mouseup" => Mouse_Event_State::Mouse_Up,
                    "mousemove" => Mouse_Event_State::Mouse_Move,
                    "contextmenu" => Mouse_Event_State::Context_Menu,
                    "wheel" => Mouse_Event_State::Wheel,
                    s => damn_it!("unrecognized mouse event: {}", s),
                },
                // movement_x: e.movement_x(),
                // movement_y: e.movement_y(),
                ctrl: e.ctrl_key(),
                shift: e.shift_key(),
                alt: e.alt_key(),
                meta: e.meta_key(),
                button: e.button(),
                buttons: Mouse_Buttons { inner: e.buttons() },
                delta_x: e.delta_x() as f32,
                delta_y: e.delta_y() as f32,
                delta_z: e.delta_z() as f32,
            },
        }
    }
    */
}
