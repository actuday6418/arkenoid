use graphics::types::Rectangle;
use std::convert::AsMut;

pub fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}

pub fn rectangles_collide(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    if rect1[0] + rect1[2] >= rect2[0]
        && rect1[0] <= rect2[0] + rect2[2]
        && rect1[1] + rect1[3] >= rect2[1]
        && rect1[1] <= rect2[1] + rect2[3]
    {
        return true;
    }
    return false;
}
