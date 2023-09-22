pub const MAP: [[u8; 8]; 8] = include!("maps/map.txt");
pub const FOV: f64 = 60.;
pub const MAX_DIST: f64 = 100.;

// standard DDA raycasting
// returns the distance to the wall
pub fn raycast(pos: (f64, f64), dir: f64, map: [[u8; 8]; 8]) -> f64 {
    let mut dist = 0.;

    let unit_step_size = (dir.to_radians().cos().recip(), dir.to_radians().sin().recip());
    let mut map_check = pos;
    let mut ray_len_1d = (0., 0.);
    let mut step = (0, 0);

    if dir > 180. && dir < 360. {
        step.0 = -1;
        ray_len_1d.0 = (map_check.0 - map_check.0.floor()) * unit_step_size.0;
    } else {
        step.0 = 1;
        ray_len_1d.0 = (map_check.0.ceil() - map_check.0) * unit_step_size.0;
    }
    if dir > 90. && dir < 270. {
        step.1 = -1;
        ray_len_1d.1 = (map_check.1 - map_check.1.floor()) * unit_step_size.1;
    } else {
        step.1 = 1;
        ray_len_1d.1 = (map_check.1.ceil() - map_check.1) * unit_step_size.1;
    }
    while dist < MAX_DIST {
        if ray_len_1d.0 < ray_len_1d.1 {
            map_check.0 += step.0 as f64;
            dist = ray_len_1d.0;
            ray_len_1d.0 += unit_step_size.0;
        } else {
            map_check.1 += step.1 as f64;
            dist = ray_len_1d.1;
            ray_len_1d.1 += unit_step_size.1;
        }
        if map_check.0 < 0. || map_check.0 > 7. || map_check.1 < 0. || map_check.1 > 7. {
            dist = MAX_DIST;
            break;
        }
        if map[map_check.0 as usize][map_check.1 as usize] != 0 {
            break;
        }
    }
    dist
}
