use raycasting::{raycast, FOV, MAP};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

mod raycasting;

const SPEED: f64 = 2.;
const TURN_SPEED: f64 = 7.;
const N_RAYS: i32 = 500;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("unnamed-game", 700, 700)
        .position_centered()
        .opengl()
        .input_grabbed()
        .build()
        .map_err(|e| e.to_string())?;

    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut player_pos = (4., 4.);
    let mut player_dir: f64 = 0.;

    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;
    let mut now = Instant::now();
    let mut then = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::W => w_pressed = true,
                    Keycode::A => a_pressed = true,
                    Keycode::S => s_pressed = true,
                    Keycode::D => d_pressed = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::W => w_pressed = false,
                    Keycode::A => a_pressed = false,
                    Keycode::S => s_pressed = false,
                    Keycode::D => d_pressed = false,
                    _ => {}
                },
                _ => {}
            }
        }
        then = now;
        now = Instant::now();
        let deltatime = now.duration_since(then).as_secs_f64();
        if w_pressed {
            player_pos.1 += player_dir.to_radians().cos() * SPEED * deltatime;
            player_pos.0 += player_dir.to_radians().sin() * SPEED * deltatime;
        }
        if s_pressed {
            player_pos.1 -= player_dir.to_radians().cos() * SPEED * deltatime;
            player_pos.0 -= player_dir.to_radians().sin() * SPEED * deltatime;
        }
        if a_pressed {
            player_pos.1 += (player_dir + 90.).to_radians().cos() * SPEED * deltatime;
            player_pos.0 += (player_dir + 90.).to_radians().sin() * SPEED * deltatime;
        }
        if d_pressed {
            player_pos.1 -= (player_dir + 90.).to_radians().cos() * SPEED * deltatime;
            player_pos.0 -= (player_dir + 90.).to_radians().sin() * SPEED * deltatime;
        }
        //set player_dir based off the relative mouse state
        let mouse_state = event_pump.relative_mouse_state();
        let mouse_x = mouse_state.x();
        player_dir -= mouse_x as f64 * TURN_SPEED * deltatime;
        if player_dir < 0. {
            player_dir = 360. + player_dir;
        }
        player_dir = player_dir % 360.;

        let size = canvas.output_size()?;
        let step = (size.0 as f64 / 8.0).round() as usize;
        
        // draw the map
        for (y, row) in MAP.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    canvas.set_draw_color(Color::WHITE);
                    canvas.fill_rect(sdl2::rect::Rect::new(
                        (x * step) as i32,
                        (y * step) as i32,
                        step as u32,
                        step as u32,
                    ))?;
                }
            }
        }

        let w = size.0 as usize;
        let h = size.1 as usize;
        let mut x = 0;
        let mut y = 0;
        canvas.set_draw_color(Color::GRAY);
        while x < w {
            canvas.draw_line((x as i32, 0), (x as i32, h as i32))?;
            x += step;
        }
        while y < h {
            canvas.draw_line((0, y as i32), (w as i32, y as i32))?;
            y += step;
        }

        // draw the player in red with half the size centered around the coordiantes
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(sdl2::rect::Rect::new(
            ((player_pos.0 * step as f64) + (step as f64) / 4.) as i32,
            ((player_pos.1 * step as f64) + (step as f64) / 4.) as i32,
            (step / 2) as u32,
            (step / 2) as u32,
        ))?;

        canvas.set_draw_color(Color::BLUE);

        // draw the player rotation line
        canvas.draw_line(
            (
                ((player_pos.0 * step as f64) + (step as f64) / 2.) as i32,
                ((player_pos.1 * step as f64) + (step as f64) / 2.) as i32,
            ),
            (
                ((player_pos.0 * step as f64)
                    + (step as f64) / 2.
                    + player_dir.to_radians().sin() * step as f64) as i32,
                ((player_pos.1 * step as f64)
                    + (step as f64) / 2.
                    + player_dir.to_radians().cos() * step as f64) as i32,
            ),
        )?;

        // draw the rays
        for i in 0..N_RAYS {
            let angle = player_dir.to_radians() - FOV.to_radians() / 2. + i as f64 * FOV.to_radians() / N_RAYS as f64;
            let dist = raycast(player_pos, angle, MAP);
            canvas.draw_line(
                (
                    ((player_pos.0 * step as f64) + (step as f64) / 2.) as i32,
                    ((player_pos.1 * step as f64) + (step as f64) / 2.) as i32,
                ),
                (
                    ((player_pos.0 * step as f64)
                        + (step as f64) / 2.
                        + angle.sin() * dist * step as f64) as i32,
                    ((player_pos.1 * step as f64)
                        + (step as f64) / 2.
                        + angle.cos() * dist * step as f64) as i32,
                ),
            )?;
        }
        
        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
