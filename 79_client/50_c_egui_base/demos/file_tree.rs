use super::*;

/*
use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Text, Widget, layout, renderer};
use iced::mouse::Cursor;
use iced::{Color, Length, Rectangle, Size, border};
/*
use iced::core::layout::{Limits, Node};
use iced::core::mouse::Cursor;
use iced::core::renderer::Style;
use iced::core::widget::Tree;
use iced::core::{Layout, Length, Rectangle, Size, Text, Widget, layout, renderer};
use iced::widget::canvas::Frame;

 */

pub enum File_Type_Label {
    Small { data: [u8; 128], len: u8 },
    Big { data: Vec<u8> },
}

impl File_Type_Label {
    pub fn new(s: &str) -> File_Type_Label {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n <= 128 {
            let mut data = [0_u8; 128];
            data[0..n].copy_from_slice(bytes);
            File_Type_Label::Small {
                data: data,
                len: n as u8,
            }
        } else {
            File_Type_Label::Big {
                data: bytes.to_vec(),
            }
        }
    }

    pub fn as_str(&self) -> &str {
        let t = match self {
            File_Type_Label::Small { data, len } => &data[0..(*len as usize)],
            File_Type_Label::Big { data } => data.as_slice(),
        };
        std::str::from_utf8(t).unwrap_or("")
    }
}

pub enum File_Tree<Msg> {
    Leaf {
        label: File_Type_Label,
        msg: Msg,
    },
    Inner {
        label: File_Type_Label,
        nodes: Vec<File_Tree<Msg>>,
        open: Cell<bool>,
    },
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for File_Tree<Message>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        layout::Node::new(Size::new(800., 800.))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let t = text!("Hello World");
        t.draw(tree, renderer, aheme, style, layout, cursor, viewport);
        button
        /*
        let mut frame = iced::widget::canvas::Frame::new(
            renderer,
            Size {
                width: 800.,
                height: 800.,
            },
        );

         */

        /*
        let canvas_text = Text {
            content: String::from("Hello, Iced Canvas!"),
            /*
            position: Point::new(50.0, 50.0),
            color: Color::BLACK,

             */
            size: 30.0.into(),
            ..Text::default()
        };

        frame.fill_text(canvas_text);

         */

        /*
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.radius),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
        */
    }
}

/*

   use iced::advanced::layout::{self, Layout};
   use iced::advanced::renderer;
   use iced::advanced::widget::{self, Widget};
   use iced::border;
   use iced::mouse;
   use iced::{Color, Element, Length, Rectangle, Size};

   pub struct Circle {
       radius: f32,
   }

   impl Circle {
       pub fn new(radius: f32) -> Self {
           Self { radius }
       }
   }

   pub fn circle(radius: f32) -> Circle {
       Circle::new(radius)
   }

   impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
   where
       Renderer: renderer::Renderer,
   {

       fn layout(
           &mut self,
           _tree: &mut widget::Tree,
           _renderer: &Renderer,
           _limits: &layout::Limits,
       ) -> layout::Node {
           layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
       }

       fn draw(
           &self,
           _tree: &widget::Tree,
           renderer: &mut Renderer,
           _theme: &Theme,
           _style: &renderer::Style,
           layout: Layout<'_>,
           _cursor: mouse::Cursor,
           _viewport: &Rectangle,
       ) {
           renderer.fill_quad(
               renderer::Quad {
                   bounds: layout.bounds(),
                   border: border::rounded(self.radius),
                   ..renderer::Quad::default()
               },
               Color::BLACK,
           );
       }
   }

   impl<Message, Theme, Renderer> From<Circle> for Element<'_, Message, Theme, Renderer>
   where
       Renderer: renderer::Renderer,
   {
       fn from(circle: Circle) -> Self {
           Self::new(circle)
       }
   }

*/

/*
#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl File_Tree {
    fn new() -> File_Tree {
        File_Tree { value: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        Column::with_children(vec![
            text(self.value).size(50).into(),
            button("Increment").on_press(Message::Increment).into(),
            text(self.value).size(50).into(),
            button("Decrement").on_press(Message::Decrement).into(),
            mouse_area(text("some other").size(20))
                .on_press(Message::Increment)
                .into(),
        ])
        .padding(20)
        .align_x(Center)
    }
}


 */


 */
