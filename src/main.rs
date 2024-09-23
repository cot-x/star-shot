#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use std::path::Path;

use c_str_macro::c_str;
use cgmath::perspective;
use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::mixer::{Music, Channel, Chunk, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

mod shader;
mod vertex;
mod stage;
mod stars;
mod ship;
mod raser;
mod enemy;
mod sprite;
mod image_manager;
mod board;

use shader::Shader;
use stage::Stage;
use stars::Stars;
use ship::Ship;
use raser::Raser;
use enemy::Enemy;
use sprite::Sprite;
use image_manager::ImageManager;
use board::Board;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;
const MAX_RASER_ENERGY: i32 = 1000;
const RASER_POWER: i32 = 30;
const CHARGE_POWER: i32 = 10;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
            if self.pos == self.data.len() {
                self.pos = 0;
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let mouse = sdl_context.mouse();
    mouse.show_cursor(false);

    let wav_file = Path::new("rsc/wav/bgm.wav");
    let desired_spec = AudioSpecDesired { freq: Some(44_100), channels: Some(2), samples: None };
    let bgm = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load WAV file");
        let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq ).expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound { data, volume: 0.25, pos: 0 }
    }).unwrap();
    bgm.resume();

    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1_024).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    Music::set_volume(32);

    let beam = Music::from_file(Path::new("rsc/wav/beam.mp3")).unwrap();

    let mut crash = Chunk::from_file(Path::new("rsc/wav/crash.mp3")).unwrap();
    crash.set_volume(32);
    let mut vanish = Chunk::from_file(Path::new("rsc/wav/vanish.mp3")).unwrap();
    vanish.set_volume(64);
    let mut dead = Chunk::from_file(Path::new("rsc/wav/dead.mp3")).unwrap();
    dead.set_volume(64);

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 1);
    //let (major, minor) = gl_attr.context_version();
    //println!("Init OpenGL: version={}.{}", major, minor);

    let window = video_subsystem
        .window("Star-Shot", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .fullscreen()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let mut image_manager = ImageManager::new();

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    let mut stage = Stage::new();
    let mut stars = Stars::new();
    let mut ship = Ship::new();
    let raser = Raser::new();
    let mut raser_energy = MAX_RASER_ENERGY;
    let mut enemies = Vec::<Enemy>::new();
    let mut sprites = Vec::<Sprite>::new();
    let mut damages = Vec::<Sprite>::new();
    let mut dead_sprite = Vec::<Sprite>::new();

    let mut board = Board::new();
    let mut score = 0u32;
    let mut interval = 0.0;
    image_manager.write_text(&score.to_string(), 50 * score.to_string().len() as u32, 50, 0, 0, 255, "score");

    let mut events = sdl_context.event_pump().unwrap();
    let mut now = ::std::time::Instant::now();
    'running: loop {
        let delay = now.elapsed().as_millis();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if raser_energy < MAX_RASER_ENERGY {
            raser_energy += CHARGE_POWER;
        }

        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let mouse_state = events.mouse_state();

            if rng.gen::<f32>() <= 0.03 {
                let mut enemy = Enemy::new();
                enemy.x = -1.0 + -2.0 * rng.gen::<f32>();
                enemy.y = -1.5 + -1.5 * rng.gen::<f32>();
                enemy.z = 20.0;
                enemies.push(enemy);
            }

            ship.x = - mouse_state.x() as f32 / WINDOW_WIDTH as f32 * 1.5 - 1.25;
            ship.y = - mouse_state.y() as f32 / WINDOW_HEIGHT as f32 * 1.25 - 1.5;
            ship.z = -2.5;
            let camera_x = -2.25 - (ship.x + 1.25) / 1.5;
            let camera_y = -2.25 - (ship.y + 1.5) / 1.5;
            let camera_z = -5.0;
            let view_matrix = Matrix4::look_at_rh(
                Point3 {
                    x: camera_x,
                    y: camera_y,
                    z: camera_z
                },
                Point3 {
                    x: -2.25,
                    y: -2.25,
                    z: 0.0
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                }
            );
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0
            );

            shader.use_program();
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            stage.draw(&shader, delay);
            stars.draw(&shader, delay);

            if let Some(i) = enemies.iter().position(|enemy| (*enemy).z < -5.0) {
                enemies.remove(i);
            }

            if ship.life > 0 && raser_energy > 0 && mouse_state.is_mouse_button_pressed(MouseButton::Left) {
                let _ = beam.play(0);
                let mut raser_z = 10.0;
                if enemies.len() != 0 {
                    let hit_z = enemies.iter().map(|enemy| enemy.intersect_z(ship.x, ship.y));
                    let hit_z = hit_z.fold(0.0/0.0, |m, v| v.min(m));
                    raser_z = if hit_z > -2.0 { hit_z } else { -2.0 };

                    if let Some(i) = enemies.iter().position(|enemy| hit_z != 10.0 && (*enemy).intersect_z(ship.x, ship.y) == hit_z) {
                        enemies[i].life = enemies[i].life - 1;
                        if enemies[i].life <= 0 {
                            let _ = Channel::all().play(&vanish, 0);
                            sprites.push(Sprite::new(10, 0.5, enemies[i].x, enemies[i].y, enemies[i].z, 0.5, 1.0, 0.5));
                            enemies.remove(i);
                            score = score + 500;
                            image_manager.delete_texture("score");
                            image_manager.write_text(&score.to_string(), 50 * score.to_string().len() as u32, 50, 0, 0, 255, "score");
                        }
                    }
                }
                raser.set(ship.x, ship.y, ship.z + 0.5, ship.x, ship.y, raser_z);
                raser.draw(&shader, delay);
                raser_energy -= RASER_POWER;
            }

            let mut enemies: Vec<_> = enemies.iter_mut().map(|enemy| { enemy.z = enemy.z - 0.1; enemy }).collect();
            enemies.iter_mut().for_each(|enemy| enemy.draw(&shader, delay));

            let mut sprites: Vec<_> = sprites.iter_mut().filter(|sprite| sprite.life > 0).collect();
            sprites.iter_mut().for_each(|sprite| sprite.draw(&shader, delay));

            if ship.life <= 0 {
                ship.z = ship.z + 10.0 * interval;
                ship.y = f32::max(ship.y - interval, -3.0);
                if ship.interval > 0 {
                    ship.interval = ship.interval - 1;
                    interval = interval + 0.01;

                    for point in ship.points() {
                        let intersect_z = enemies.iter().map(|enemy| enemy.intersect_z(point.x, point.y));
                        for z in intersect_z {
                            if point.z - 0.01 < z && point.z + 0.01 > z {
                                let _ = Channel::all().play(&crash, 0);
                                damages.push(Sprite::new(5, 0.01, point.x, point.y, point.z, 1.0, 0.75, 0.5));
                            }
                        }
                    }

                    ship.draw(&shader, delay);

                    let mut damages: Vec<_> = damages.iter_mut().filter(|sprite| sprite.life > 0).collect();
                    damages.iter_mut().for_each(|sprite| sprite.draw(&shader, delay));
                } else if ship.interval == 0 {
                    let _ = Channel::all().play(&dead, 0);
                    dead_sprite.push(Sprite::new(10, 0.5, ship.x, ship.y, ship.z, 1.0, 1.0, 0.0));
                    ship.interval = -1;
                }
            } else {
                for point in ship.points() {
                    let intersect_z = enemies.iter().map(|enemy| enemy.intersect_z(point.x, point.y));
                    for z in intersect_z {
                        if point.z - 0.01 < z && point.z + 0.01 > z {
                            let _ = Channel::all().play(&crash, 0);
                            damages.push(Sprite::new(5, 0.01, point.x, point.y, point.z, 1.0, 0.75, 0.5));
                            ship.life = ship.life - 1;
                        }
                    }
                }
                ship.draw(&shader, delay);

                let mut damages: Vec<_> = damages.iter_mut().filter(|sprite| sprite.life > 0).collect();
                damages.iter_mut().for_each(|sprite| sprite.draw(&shader, delay));

                score = score + 1;
                image_manager.delete_texture("score");
                image_manager.write_text(&score.to_string(), 50 * score.to_string().len() as u32, 50, 0, 0, 255, "score");
            }

            let mut dead_sprite: Vec<_> = dead_sprite.iter_mut().filter(|sprite| sprite.life > 0).collect();
            dead_sprite.iter_mut().for_each(|sprite| sprite.draw(&shader, delay));

            let texture_id = image_manager.get_texture_id("score");
            gl::BindTexture(gl::TEXTURE_2D, texture_id as u32);
            board.x = -5.5;
            board.y = -1.0;
            board.width = 1.5;
            board.height = 0.5;
            board.draw(&shader, delay);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            window.gl_swap_window();
        }

        now = ::std::time::Instant::now();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
