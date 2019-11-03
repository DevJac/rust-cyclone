use cyclone::particle::Particle;
use cyclone::vec::Vec3;
use num::clamp;
use rand::prelude::*;
use rand_distr::StandardNormal;
use raylib::prelude::*;

const ZERO: Vec3<f32> = Vec3(0.0, 0.0, 0.0);

const UP: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn c_to_r(v: Vec3<f32>) -> Vector3 {
    Vector3 {
        x: v.0,
        y: v.1,
        z: v.2,
    }
}

struct Spark {
    particle: Particle<f32>,
    life: f32,
    age: f32,
}

fn add_sparks(n_sparks: i32, sparks: &mut Vec<Spark>) {
    for _ in 1..=n_sparks {
        let spark = Spark {
            particle: Particle {
                position: ZERO,
                velocity: Vec3(
                    thread_rng().sample::<f32, _>(StandardNormal) * 20.0,
                    thread_rng().sample::<f32, _>(StandardNormal) * 20.0,
                    thread_rng().sample::<f32, _>(StandardNormal) * 20.0,
                ),
                acceleration: ZERO,
                damping: 0.5,
                inverse_mass: 1.0,
            },
            life: thread_rng().sample::<f32, _>(StandardNormal) * 2.0 + 8.0,
            age: 0.0,
        };
        sparks.push(spark);
    }
}

fn integrate_sparks(duration: f32, sparks: &mut Vec<Spark>) {
    for spark in sparks {
        spark.age += duration;
        spark.particle.acceleration = Vec3(0.0, -10.0, 0.0);
        spark.particle.integrate(duration);
    }
}

fn main() {
    let mut sparks: Vec<Spark> = Vec::new();
    let (mut rl, thread) = raylib::init().size(800, 450).title("Fireworks").build();
    rl.set_target_fps(60);
    let camera = Camera::perspective(vec3(0.0, 50.0, 100.0), vec3(0.0, 0.0, 0.0), UP, 45.0);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            add_sparks(100, &mut sparks);
        }
        integrate_sparks(rl.get_frame_time(), &mut sparks);
        sparks.retain(|spark| spark.age < spark.life);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
        d.draw_text("Press SPACE for Fireworks!", 100, 10, 20, Color::GRAY);
        let mut d3 = d.begin_mode_3D(camera);
        for spark in &sparks {
            let life = clamp(1.0 - (spark.age / spark.life), 0.0, 1.0);
            d3.draw_sphere(c_to_r(spark.particle.position), life, Color::GOLD);
        }
    }
}
