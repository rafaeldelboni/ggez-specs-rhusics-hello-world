extern crate ggez;
extern crate specs;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shrev;
extern crate cgmath;

mod systems;
mod components;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use specs::{Dispatcher, DispatcherBuilder, Builder, World, RunNow};

use rhusics_core::{RigidBody, Pose};
use rhusics_ecs::{DeltaTime, WithRigidBody};
use rhusics_ecs::collide2d::{
    BroadBruteForce2,
    GJK2,
    BodyPose2,
    CollisionStrategy,
    CollisionShape2,
    CollisionMode};
use rhusics_ecs::physics2d::{
    ContactEvent2, ContactResolutionSystem2, CurrentFrameUpdateSystem2,
    NextFrameSetupSystem2, SpatialCollisionSystem2,
    SpatialSortingSystem2, Rectangle, Mass2, Velocity2};
use cgmath::{Basis2, One, Point2, Vector2};
use shrev::EventChannel;

use systems::{ControlSystem, RenderingSystem, MoveSystem};
use components::{Controlable, Square, Velocity};

struct MainState<'a, 'b> {
    frames: usize,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);

        let mut world = World::new();
        world.register::<Square>();
        world.register::<Velocity>();
        world.register::<Controlable>();

        let mut impulse_solver = CurrentFrameUpdateSystem2::<f32, BodyPose2<f32>>::new();
        let mut next_frame = NextFrameSetupSystem2::<f32, BodyPose2<f32>>::new();
        let mut sort = SpatialSortingSystem2::<f32, BodyPose2<f32>, ()>::new();
        let mut collide = SpatialCollisionSystem2::<f32, BodyPose2<f32>, ()>::new()
                    .with_broad_phase(BroadBruteForce2::default())
                    .with_narrow_phase(GJK2::new());
        let mut contact_resolution = ContactResolutionSystem2::<f32, BodyPose2<f32>>::new();

        impulse_solver.setup(&mut world.res);
        next_frame.setup(&mut world.res);
        sort.setup(&mut world.res);
        collide.setup(&mut world.res);
        contact_resolution.setup(&mut world.res);

        world.write_resource::<EventChannel<ContactEvent2<f32>>>()
            .register_reader();

        world
            .create_entity()
            .with_dynamic_rigid_body(
                CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                    CollisionStrategy::FullResolution,
                    CollisionMode::Discrete,
                    Rectangle::new(100., 100.).into(),
                ),
                BodyPose2::new(Point2::new(10., 10.), Basis2::one()),
                Velocity2::new(Vector2::new(0.0, 0.0), 0.0),
                RigidBody::default(),
                Mass2::new(1.),
            )
            .with(Square {
                body_shape: graphics::Point2::new(100.0, 100.0),
                position: graphics::Point2::new(10.0, 10.0)})
            .build();

        world
            .create_entity()
            .with_dynamic_rigid_body(
                CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                    CollisionStrategy::FullResolution,
                    CollisionMode::Discrete,
                    Rectangle::new(400., 100.).into(),
                ),
                BodyPose2::new(Point2::new(20., 200.), Basis2::one()),
                Velocity2::new(Vector2::new(5.0, 5.0), 0.0),
                RigidBody::default(),
                Mass2::new(1.),
            )
            .with(Square {
                body_shape: graphics::Point2::new(400.0, 100.0),
                position: graphics::Point2::new(20.0, 200.0)})
            .with(Velocity { x: 5., y: 5. })
            .build();

        world
            .create_entity()
            .with_dynamic_rigid_body(
                CollisionShape2::<f32, BodyPose2<f32>, ()>::new_simple(
                    CollisionStrategy::FullResolution,
                    CollisionMode::Discrete,
                    Rectangle::new(200., 100.).into(),
                ),
                BodyPose2::new(Point2::new(20., 400.), Basis2::one()),
                Velocity2::new(Vector2::new(0.0, 0.0), 0.0),
                RigidBody::default(),
                Mass2::new(1.),
            )
            .with(Square {
                body_shape: graphics::Point2::new(200.0, 100.0),
                position: graphics::Point2::new(20.0, 400.0)})
            .with(Velocity { x: 0., y: 0. })
            .with(Controlable)
            .build();

        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(
                MoveSystem,
                "move_system",
                &[],
            )
            .with(
                impulse_solver,
                "solver",
                &[],
            )
            .with(
                next_frame,
                "next_frame",
                &["solver"],
            )
            .with(
                sort,
                "sorting",
                &["next_frame"],
            )
            .with(
                collide,
                "collision",
                &["sorting"],
            )
            .with(
                contact_resolution,
                "resolution",
                &["collision"],
            )
            .build();

        let state = MainState {
            frames: 0,
            world,
            dispatcher,
        };

        Ok(state)
    }
}

impl<'a, 'b> event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::get_delta(ctx);
        let seconds = (dt.as_secs() as f32 + (dt.subsec_nanos() as f32 / 1_000_000_000.0)).min(1.0 / 20.0);
        self.world.write_resource::<DeltaTime<f32>>().delta_seconds = seconds;
        self.dispatcher.dispatch(&self.world.res);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        {
            let mut rs = RenderingSystem::new(ctx);
            rs.run_now(&mut self.world.res);
        }

        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool
    ) {
        let mut cs = ControlSystem::new(keycode, true);
        cs.run_now(&mut self.world.res);
    }

    fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: event::Keycode,
        _keymod: event::Mod,
        _repeat: bool

    ) {
        let mut cs = ControlSystem::new(keycode, false);
        cs.run_now(&mut self.world.res);
    }
}

fn main() {
    let c = conf::Conf::new();
    println!("Starting with default config: {:#?}", c);

    let ctx = &mut Context::load_from_conf("ggez-specs-hello-world", "ggez", c).unwrap();

    match MainState::new(ctx) {
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
    }
}
