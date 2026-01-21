use e_api::instant::{Duration, Instant};
use e_api::*;

use crate::egui_ltree_lib::{
    NodeConfig, TreeView, TreeViewBuilder, TreeViewState,
};
use egui::{Label, NumExt, ThemePreference};
// use uuid::Uuid;

pub struct MyApp {
    tree: LTree_Node,
    state: TreeViewState<String>,
}
impl MyApp {
    pub fn new(tree: LTree_Node) -> Self {
        // let tree = build_tree(100_000, 1, 2);
        let mut state = TreeViewState::default();
        init_state(&mut state, &tree);
        MyApp { tree, state }
    }
}

fn init_state(state: &mut TreeViewState<String>, node: &LTree_Node) {
    let LTree_Node::Directory { name, children } = node else {
        return;
    };
    state.set_openness(name.clone(), true);
    for child in children {
        init_state(state, child);
    }
}

impl MyApp {
    pub fn update(&mut self, ui: &mut egui::Ui) {
        use c_app_msg::*;
        egui::ScrollArea::both().show(ui, |ui| {
            let (response, actions) =
                TreeView::new(ui.make_persistent_id("Names tree view"))
                    .show_state(ui, &mut self.state, |builder| {
                        build_node_once(&self.tree, builder);
                    });

            for x in actions {
                match x {
                    crate::egui_ltree_lib::Action::SetSelected(items) => {
                        wlog!("set selected: {:?}", items);
                        if items.len() > 0 {
                            G_CmsgQ::send_oneshot(Cmsg_Inner::Ww_sheet(
                                Cmsg_WwSheet::OpenFile {
                                    cmd: items[0].to_string(),
                                },
                            ));
                        }
                    }
                    crate::egui_ltree_lib::Action::Move(drag_and_drop) => {
                        //
                    }
                    crate::egui_ltree_lib::Action::Drag(drag_and_drop) => {
                        //
                    }
                    crate::egui_ltree_lib::Action::Activate(activate) => {
                        //
                    }
                    crate::egui_ltree_lib::Action::DragExternal(
                        drag_and_drop_external,
                    ) => {}
                    crate::egui_ltree_lib::Action::MoveExternal(
                        drag_and_drop_external,
                    ) => {}
                }
            }
        });
    }
}

fn build_node_once(node: &LTree_Node, builder: &mut TreeViewBuilder<String>) {
    build_node_once_inner(node, builder);
}

fn build_node_once_inner(
    node: &LTree_Node,
    builder: &mut TreeViewBuilder<String>,
) {
    match node {
        LTree_Node::Directory { children, name } => {
            builder.node(DefaultNode {
                name,
                is_dir: true,
                full_name: &"".to_string(),
            });
            builder.close_dir_in(children.len());
            for child in children {
                build_node_once_inner(child, builder);
            }
        }
        LTree_Node::Leaf { name, full_name } => {
            builder.node(DefaultNode {
                name,
                full_name,
                is_dir: false,
            });
        }
    }
}

#[derive(Debug)]
pub enum LTree_Node {
    Directory {
        children: Vec<LTree_Node>,
        name: String,
    },
    Leaf {
        name: String,
        full_name: String,
    },
}

struct DefaultNode<'a> {
    name: &'a String,
    full_name: &'a String,
    is_dir: bool,
}

impl<'a> NodeConfig<String> for DefaultNode<'a> {
    fn id(&self) -> &String {
        &self.full_name
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }

    fn default_open(&self) -> bool {
        true
    }

    fn label(&mut self, ui: &mut egui::Ui) {
        ui.add(Label::new(self.name).selectable(false));
    }
}
