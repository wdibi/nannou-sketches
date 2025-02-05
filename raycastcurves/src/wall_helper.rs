use crate::mondrian::split_squares;
pub use crate::mondrian::Square;
use crate::Element;
use crate::scene::Curve;
//use crate::Element::Curve;
use crate::scene::Material;
use crate::scene::SurfaceType;
use nannou::prelude::*;
use nannou_ray2d::BoundingVolume;

pub fn make_walls(
    walls: &mut Vec<Element>,
    win: &geom::Rect,
    tile_count_w: u32,
    wall_split: f32,
    perc_padding: f32,
    hole_pct: f32,
    hole_n: usize,
    color_a: Rgba,
    color_b: Rgba,
    material: &Material,
) {
    walls.clear();
    let margin: i32 = 100;
    let step = (win.w() as f32) as u32 / tile_count_w;

    let mut squares: Vec<Square> = Vec::new();
    squares.push(Square {
        x: win.left() + (margin as f32 / 2.0),
        y: win.bottom() + (margin as f32 / 2.0),
        width: (win.w() - margin as f32),
        height: (win.h() - margin as f32),
    });
    for i in (win.left() as i32..win.right() as i32).step_by(step as usize) {
        split_squares(i as f32, i as f32, &mut squares, wall_split);
    }
    for square in &squares {
        let padding = step as f32 * perc_padding;
        create_curve_from_square(&square, padding, hole_pct, hole_n, walls, &material);
    }
    change_color_walls(walls, color_a, color_b);
}

pub fn change_color_walls(walls: &mut Vec<Element>, color_a: Rgba, color_b: Rgba) {
    walls.iter_mut().for_each(|element| {
        let color: Rgba = if random_range(0.0, 1.0) > 0.5 {
            color_a
        } else {
            color_b
        };
        element.material_mut().coloration = color;
    });
}

pub fn change_surface_walls(walls: &mut Vec<Element>, surface: &SurfaceType) {
    walls.iter_mut().for_each(|element| {
        element.material_mut().surface = surface.clone();
    });
}

pub fn create_curve_from_square(
    square: &Square,
    padding: f32,
    hole: f32,
    hole_n: usize,
    walls: &mut Vec<Element>,
    mat: &Material,
) {
    let center = vec2(
        square.x + square.width / 2.0,
        square.y + square.height / 2.0,
    );
    let mut points = vec![];

    let mut wall_length = 360;
    if hole_n > 0 {
        wall_length = 360 / hole_n;
    }

    let pad = (wall_length as f32 * hole) as usize;
    let mut start_from = 0;
    let mut end_to = start_from + wall_length - pad;
    let cloned_mat = mat.clone();
    let bounding_volume = get_circle_from_square(square, &padding);

    if hole > 0.1 {
        for i in (0..=360).step_by(1) {
            let rad = deg_to_rad(i as f32);
            //points.push(center + vec2(rad.sin() * radius, rad.cos() * radius));
            let x = (square.width / 2.0 - padding) * rad.cos();
            let y = (square.height / 2.0 - padding) * rad.sin();

            if i >= start_from && i < end_to {
                points.push(center + vec2(x, y))
            }

            if i == end_to {
                points.push(center + vec2(x, y));

                walls.push(Element::Curve(Curve{
                    points: points.clone(),
                    material: cloned_mat,
                    ray_anchor_point: Some(center),
                    bounding_volume: Some(bounding_volume),
                }));
                points.clear();
                start_from = i + pad;
                end_to = start_from + wall_length - pad;
            }
        }
    } else {
        for i in (0..=360).step_by(1) {
            let rad = deg_to_rad(i as f32);
            //points.push(center + vec2(rad.sin() * radius, rad.cos() * radius));
            let x = (square.width / 2.0 - padding) * rad.cos();
            let y = (square.height / 2.0 - padding) * rad.sin();
            points.push(center + vec2(x, y))
        }
        walls.push(Element::Curve(Curve{
            points: points.clone(),
            material: cloned_mat,
            ray_anchor_point: Some(center),
            bounding_volume: Some(bounding_volume),
        }));
        points.clear();
    }
}

// TODO, use an aabb instead of a circle for this case
fn get_circle_from_square(square: &Square, padding: &f32) -> BoundingVolume {
    let radius: f32;
    if square.width > square.height {
        radius = square.width / 2.0 - padding;
    } else {
        radius = square.height / 2.0 - padding;
    }
    let center = vec2(
        square.x + square.width / 2.0,
        square.y + square.height / 2.0,
    );
    BoundingVolume::Circle {
        position: center,
        radius,
    }
}
