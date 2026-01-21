use super::*;

pub struct Capp_Vid__Id_Slide {}

impl Capp_Vid__Id_Slide {
    pub fn to_anyview(t: Cmsg_Vid__Id_Slide) -> AnyView {
        wlog!("Capp_Vid__Id_Slide::to_anyview");
        match t {
            Cmsg_Vid__Id_Slide::Start => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::None => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_got_audio_permission => view! {
                <ol style:font-size="24px" >
                <li> <button style:font-size="24px" > SQL 101 </button> </li>
                <li> <button style:font-size="24px" > Python 101 </button> </li>
                <li> <button style:font-size="24px" > Financial Modelling 101 </button> </li>
                </ol>
            }
            .into_any(),
            Cmsg_Vid__Id_Slide::title_0 => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_exception => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_good_jogb => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_infinite_loop => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_not_42 => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_one_more_left => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_repeat => view! {}.into_any(),
            Cmsg_Vid__Id_Slide::title_two_more_to_go => view! {}.into_any(),
        }
    }
}
