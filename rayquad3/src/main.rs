use edapx_colors::Palette;
use nannou::prelude::*;
use nannou::ui::prelude::*;
//use ray2d::BouncingRay2D;

mod bouncing;
pub use crate::bouncing::BouncingRay2D;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    walls: Vec<Vector2>,
    rays: Vec<BouncingRay2D>,
    draw_gui: bool,
    ui: Ui,
    ids: Ids,
    ray_width: f32,
    wall_width: f32,
    rotation: f32,
    scheme_id: usize,
    palette: Palette,
    tile_count_w: u32,
}

widget_ids! {
    struct Ids {
        ray_width,
        wall_width,
        rotation,
        scheme_id
    }
}

fn model(app: &App) -> Model {
    let tile_count_w = 12;
    app.new_window()
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut walls: Vec<Vector2> = Vec::new();
    let mut rays: Vec<BouncingRay2D> = Vec::new();
    let win = app.window_rect();

    let draw_gui = true;

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let ray_width = 6.0;
    let wall_width = 2.0;
    let rotation = 0.0;

    let scheme_id = 0;
    let palette = Palette::new();
    make_walls(&mut walls, &mut rays, &win, tile_count_w);

    Model {
        walls,
        rays,
        draw_gui,
        ui,
        ids,
        ray_width,
        wall_width,
        rotation,
        scheme_id,
        palette,
        tile_count_w,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();
    {
        fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
            widget::Slider::new(val, min, max)
                .w_h(200.0, 30.0)
                .label_font_size(15)
                .rgb(0.3, 0.3, 0.3)
                .label_rgb(1.0, 1.0, 1.0)
                .border(0.0)
        }

        for value in slider(model.wall_width as f32, 1.0, 15.0)
            .top_left_with_margin(20.0)
            .label("wall width")
            .set(model.ids.wall_width, ui)
        {
            model.wall_width = value;
        }

        for value in slider(model.ray_width, 1.0, 10.0)
            .down(10.0)
            .label("ray width")
            .set(model.ids.ray_width, ui)
        {
            model.ray_width = value;
        }

        for value in slider(model.rotation, -PI, PI)
            .down(10.0)
            .label("Rotation")
            .set(model.ids.rotation, ui)
        {
            model.rotation = value;
        }

        for value in slider(model.scheme_id as f32, 0.0, 5.0)
            .down(10.0)
            .label("scheme_id")
            .set(model.ids.scheme_id, ui)
        {
            model.scheme_id = value as usize;
        }
    }

    for r in model.rays.iter_mut() {
        r.collisions.clear();

        while !r.max_bounces_reached() {
            let mut collision: Vector2 = vec2(0.0, 0.0);
            let mut distance: f32 = Float::infinity();
            let mut surface_normal: Vector2 = vec2(0.0, 0.0);
            // find the closest intersection point between the ray and the walls
            for index in (0..model.walls.len()).step_by(2) {
                if let Some(collision_distance) = r.ray.intersect_segment(
                    model.walls[index].x,
                    model.walls[index].y,
                    model.walls[index + 1].x,
                    model.walls[index + 1].y,
                ) {
                    if collision_distance < distance {
                        distance = collision_distance;
                        collision = r.ray.orig
                            + r.ray
                                .dir
                                .with_magnitude(collision_distance - model.wall_width / 2.0);
                        let segment_dir = (model.walls[index] - model.walls[index + 1]).normalize();
                        surface_normal = vec2(segment_dir.y, -segment_dir.x);
                    }
                }
            }
            if distance < Float::infinity() {
                // collision point
                r.bounces += 1;
                let refl = r.ray.reflect(surface_normal);
                r.ray.orig = collision + refl.with_magnitude(0.001);
                r.ray.dir = refl;
                r.collisions.push(collision);
            } else {
                break;
            };
        }
        r.reset();
        //r.ray.dir = r.ray.dir.rotate(model.rotation);
        r.ray.dir = r.ray.dir.rotate(_app.time * 0.001);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(model.palette.get_scheme(model.scheme_id)[0]);

    // draw the walls
    let size = model.walls.len();
    for index in (0..size).step_by(2) {
        draw.line()
            .weight(model.wall_width)
            .color(model.palette.get_scheme(model.scheme_id)[1])
            .start(model.walls[index])
            .caps_round()
            .end(model.walls[index + 1]);
    }

    for r in &model.rays {
        draw.arrow()
            .color(model.palette.get_scheme(model.scheme_id)[3])
            .start(r.ray.orig)
            .end(r.ray.orig + r.ray.dir.with_magnitude(100.0));
        for c in &r.collisions {
            draw.ellipse()
                .x_y(c.x, c.y)
                .w_h(10., 10.)
                .color(model.palette.get_scheme(model.scheme_id)[2]);
        }
        let ppp = r
            .collisions
            .iter()
            .map(|v| (pt2(v.x, v.y), model.palette.get_scheme(model.scheme_id)[2]));
        draw.polygon()
            //.points(r.collisions.iter().cloned())
            .points_colored(ppp)
            //.weight(3.0)
            //.weight(model.ray_width)
            //.caps_round()
            .color(model.palette.get_scheme(model.scheme_id)[2]);
    }

    draw.to_frame(app, &frame).unwrap();

    if model.draw_gui {
        model.ui.draw_to_frame(app, &frame).unwrap();
    }
}

fn make_walls(
    walls: &mut Vec<Vector2>,
    rays: &mut Vec<BouncingRay2D>,
    win: &geom::Rect,
    tile_count_w: u32,
) {
    let side = win.w() as u32 / tile_count_w;
    let mut xpos = win.left();
    let mut ypos = win.bottom();

    for _x in 0..tile_count_w {
        for _y in 0..(win.h() as u32 / side as u32) {
            let coin = random_range(0.0, 1.0);
            let start_p;
            let end_p;
            let padding = 0.1 * side as f32;
            if coin > 0.5 {
                start_p = vec2(xpos + padding, ypos + side as f32 - padding);
                end_p = vec2(xpos + side as f32 - padding, ypos + padding);
            } else {
                start_p = vec2(xpos + padding, ypos + padding);
                end_p = vec2(xpos + side as f32 - padding, ypos + side as f32 - padding);
            }

            //if _x % 2 == 0 && _y % 2 == 0 {
            // let mut r = BouncingRay2D::new();
            // r.ray_origin.dir = Vector2::from_angle(random_range(-PI, PI));
            // r.ray_origin.orig = start_p;
            // r.ray.orig = start_p;
            // rays.push(r);
            //} else {
            walls.push(start_p);
            walls.push(end_p);
            //}

            ypos += side as f32;
        }
        ypos = win.bottom();
        xpos += side as f32;
    }
    let mut r = BouncingRay2D::new();
    r.ray_origin.dir = Vector2::from_angle(random_range(-PI, PI));
    r.ray_origin.orig = vec2(0.0, 0.0);
    r.ray.orig = vec2(0.0, 0.0);
    rays.push(r);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.time.to_string() + ".png");
            //.capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::G => model.draw_gui = !model.draw_gui,
        _other_key => {}
    }
}
