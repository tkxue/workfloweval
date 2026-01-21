use super::*;

#[derive(Serialize, Deserialize, JsData, Debug)]
pub enum Cmsg_HVid {
    PlayVid(Cmsg_Vid__Id_Slide),
}

#[repr(u16)]
#[derive(BigEnum, Clone, Copy, Serialize, Deserialize)]
pub enum Cmsg_Vid__Id_Slide {
    Start,
    None,
    title_got_audio_permission,
    title_0,
    title_exception,
    title_good_jogb,
    title_infinite_loop,
    title_not_42,
    title_one_more_left,
    title_repeat,
    title_two_more_to_go,
}

impl T_BigEnum for Cmsg_Vid__Id_Slide {}
