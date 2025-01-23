use std::f32::consts::PI;

#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

fn rotate_point(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let angle = angle % 360.0;
    let angle_rad = deg_to_rad(angle);
    let x_rot = x * angle_rad.cos() - y * angle_rad.sin();
    let y_rot = x * angle_rad.sin() + y * angle_rad.cos();
    (x_rot, y_rot)
}

pub fn get_rotated_corners(rect: &Rectangle) -> Vec<(f32, f32)> {
    let hw = rect.width / 2.0;
    let hh = rect.height / 2.0;

    let corners = vec![
        (-hw, -hh),
        (hw, -hh),
        (hw, hh),
        (-hw, hh),
    ];
    let rotation = rect.rotation % 360.0;

    corners
        .into_iter()
        .map(|(x, y)| {
            let (x_rot, y_rot) = rotate_point(x, y, rotation);
            (x_rot + rect.x, y_rot + rect.y)
        })
        .collect()
}

fn project_point_on_axis(x: f32, y: f32, axis: (f32, f32)) -> f32 {
    x * axis.0 + y * axis.1
}

fn projections_overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    max1 >= min2 && max2 >= min1
}

pub fn check_collision(rot_rect: &Rectangle, non_rot_rect: &Rectangle) -> bool {
    let rot_corners = get_rotated_corners(rot_rect);
    let non_rot_corners = vec![
        (-non_rot_rect.width / 2.0 + non_rot_rect.x, -non_rot_rect.height / 2.0 + non_rot_rect.y),
        (non_rot_rect.width / 2.0 + non_rot_rect.x, -non_rot_rect.height / 2.0 + non_rot_rect.y),
        (non_rot_rect.width / 2.0 + non_rot_rect.x, non_rot_rect.height / 2.0 + non_rot_rect.y),
        (-non_rot_rect.width / 2.0 + non_rot_rect.x, non_rot_rect.height / 2.0 + non_rot_rect.y),
    ];

    let mut axes = Vec::new();

    for i in 0..4 {
        let x1 = rot_corners[i].0;
        let y1 = rot_corners[i].1;
        let x2 = rot_corners[(i + 1) % 4].0;
        let y2 = rot_corners[(i + 1) % 4].1;

        let axis = (y2 - y1, x1 - x2);
        axes.push(axis);
    }

    for i in 0..4 {
        let x1 = non_rot_corners[i].0;
        let y1 = non_rot_corners[i].1;
        let x2 = non_rot_corners[(i + 1) % 4].0;
        let y2 = non_rot_corners[(i + 1) % 4].1;

        let axis = (y2 - y1, x1 - x2);
        axes.push(axis);
    }

    for axis in axes {
        let mut min_rot = f32::INFINITY;
        let mut max_rot = f32::NEG_INFINITY;
        for corner in &rot_corners {
            let projection = project_point_on_axis(corner.0, corner.1, axis);
            min_rot = min_rot.min(projection);
            max_rot = max_rot.max(projection);
        }

        let mut min_non_rot = f32::INFINITY;
        let mut max_non_rot = f32::NEG_INFINITY;
        for corner in &non_rot_corners {
            let projection = project_point_on_axis(corner.0, corner.1, axis);
            min_non_rot = min_non_rot.min(projection);
            max_non_rot = max_non_rot.max(projection);
        }

        if !projections_overlap(min_rot, max_rot, min_non_rot, max_non_rot) {
            return false;
        }
    }
    true

}
