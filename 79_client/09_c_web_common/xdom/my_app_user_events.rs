use super::*;

use bitflags::*;

#[derive(Debug)]
pub enum My_App_User_Events {
    Resize { width: u32, height: u32 },
    Pointer_Lock_Lost,
}

pub enum Xgpu_Window_Event {
    ReceivedCharacter(char),

    ActivationTokenDone {
        /*
        serial: AsyncRequestSerial,
        token: ActivationToken,

         */
    },
    /*
    Resized(Xgpu_Physical_Size<u32>),
    Moved(Xgpu_Physical_Position<i32>),

     */
    CloseRequested,
    Destroyed,
    DroppedFile(/*PathBuf */),
    HoveredFile(/* PathBuf */),
    HoveredFileCancelled,
    Focused(bool),

    /*
    KeyboardInput {
        event: Xgpu_Event,
        /*
        device_Id: DeviceId,
        is_synthetic: bool,

         */
    },

     */
    ModifiersChanged(/* Modifiers */),
    Ime(/* Ime */),
    /*
        CursorMoved {
            positionN: Xgpu_Physical_Position<f64>, /*
                                                    device_Id: DeviceId,

                                                     */
        },
    */
    CursorEntered {
        /*
        device_Id: DeviceId,

         */
    },
    CursorLeft {
        /*
        device_Id: DeviceId,

         */
    },
    MouseWheel {
        /*
        device_Id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,

         */
    },
    MouseInput {
        /*
        device_Id: DeviceId,
        state: ElementState,
        button: MouseButton,

         */
    },
    TouchpadMagnify {
        /*
        device_Id: DeviceId,
        delta: f64,
        phase: TouchPhase,

         */
    },
    SmartMagnify {
        /*
        device_Id: DeviceId,

         */
    },
    TouchpadRotate {
        /*
        device_Id: DeviceId,
        delta: f32,
        phase: TouchPhase,

         */
    },
    TouchpadPressure {
        /*
        device_Id: DeviceId,
        pressure: f32,
        stage: i64,

         */
    },
    AxisMotion {
        /*
        device_Id: DeviceId,
        axis: AxisId,
        value: f64,

         */
    },
    Touch(
        /*
        Touch

         */
    ),
    ScaleFactorChanged {
        scale_factor: f64,
        /*
        inner_size_writer: InnerSizeWriter,

         */
    },
    ThemeChanged(
        /*
        Theme

         */
    ),
    Occluded(bool),
    RedrawRequested,
}

pub enum Xgpu_Device_Event {
    Added,
    Removed,
    MouseMotion { delta: (f64, f64) },
    MouseWheel {/* delta: MouseScrollDelta */},
    Motion {/* axis: AxisId, value: f64 */},
    Button {/* button: ButtonId, state: ElementState */},
    Key(/* RawKeyEvent */),
}

pub enum Xgpu_Gfx_Event<'a> {
    NewEvents(/* StartCause */),
    WindowEvent {
        /* window_Id: WindowId, event: WindowEvent  */
        event: Xgpu_Window_Event,
    },
    DeviceEvent {
        /* device_Id: DeviceId,
         */
        event: Xgpu_Device_Event,
    },
    UserEvent(&'a My_App_User_Events),
    Suspended,
    Resumed,
    MainEventsCleared,
    RedrawRequested,
    LoopDestroyed,
    RedrawEventsCleared,
}

pub struct Crate_gfx_handle_api {}
