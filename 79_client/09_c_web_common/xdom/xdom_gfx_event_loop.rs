use super::*;

pub trait Xgpu_Gfx_Event_Loop_Fwd_T: Into_Rc_Any_T {
    fn send_event(&self, x: My_App_User_Events) -> Result<(), ()>;
    fn run_loop(&self, cb: Rc<dyn Fn(&Xgpu_Gfx_Event)>);
    fn send_resize_event(&self);
}
