use crate::math::vector::common::Vector3;

pub struct Viewport {
    pub position: Vector3,
    pub euler: Vector3,

    pub field_of_view: f32,
    pub near_plane: f32,
    pub far_plane: f32
}