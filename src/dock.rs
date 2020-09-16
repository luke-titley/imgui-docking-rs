use crate::sys;
use crate::Direction;
use crate::Ui;

#[derive(Clone, Copy, PartialEq)]
pub struct DockNode {
    id: sys::ImGuiID,
}

impl DockNode {
    pub fn new(id: sys::ImGuiID) -> Self {
        Self { id }
    }

    pub fn size(mut self, size: [f32; 2]) -> Self {
        unsafe { sys::igDockBuilderSetNodeSize(self.id, sys::ImVec2::from(size)) }

        self
    }

    pub fn position(mut self, size: [f32; 2]) -> Self {
        unsafe { sys::igDockBuilderSetNodePos(self.id, sys::ImVec2::from(size)) }

        self
    }

    pub fn split<F: FnOnce(DockNode)>(
        mut self,
        split_dir: Direction,
        size_ratio: f32,
        dir: F,
        opposite_dir: F,
    ) {
        let mut out_id_at_dir: sys::ImGuiID = 0;
        let mut out_id_at_opposite_dir: sys::ImGuiID = 0;

        unsafe {
            sys::igDockBuilderSplitNode(
                self.id,
                split_dir as i32,
                size_ratio,
                &mut out_id_at_dir,
                &mut out_id_at_opposite_dir,
            );
        }

        dir(DockNode::new(out_id_at_dir));
        opposite_dir(DockNode::new(out_id_at_opposite_dir));
    }
}

pub struct Dock {}

impl Dock {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<F: FnOnce(DockNode)>(self, f: F) {
        let dock_id = unsafe { sys::igDockBuilderAddNode(0, sys::ImGuiDockNodeFlags_None as i32) };
        f(DockNode::new(dock_id));
        unsafe { sys::igDockBuilderFinish(dock_id) };
    }
}
