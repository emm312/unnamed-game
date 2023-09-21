pub const MAP: [[u8; 8]; 8] = include!("maps/map.txt");
pub const FOV: f64 = 60.0;

// standard DDA raycasting
// returns the distance to the wall
pub fn raycast(player_pos: (f64, f64), angle: f64, map: [[u8; 8]; 8]) -> f64 {
    let dist = (player_pos.0-player_pos.0.floor(), player_pos.1-player_pos.1.floor());
    let slope = angle.to_radians().tan();
    let y_inc_per_x = slope;
    let x_inc_per_y = slope.recip();
    loop {

    }
    0.
}
