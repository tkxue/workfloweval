use super::*;

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug, JsData)]
pub enum Id_GActor {
    H_Gfx,
    H_Index,
    H_Vid,

    Gfx_Xgpu_Shader,

    Ww_Net,
    Ww_Editor,

    Ww0,
    Ww1,
    Ww2,
    Ww3,
    Ww4,

    Ww_Desktop,
    Ww_Map_Fast,
    Ww_Map_Slow,
    Ww_Map_Data,
}
