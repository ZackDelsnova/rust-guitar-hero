use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;

/// Your main game state
struct GameState {
    // Example variable for note position (falls down screen)
    note_y: f32,

    // TODO: Add your systems here
    // audio_manager: AudioManager,
    // note_spawner: NoteSpawner,
    // input_manager: InputManager,
    // score_manager: ScoreManager,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState {
            note_y: 0.0, // start at top
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        // Example note falling (replace with NoteSpawner + active notes update)
        self.note_y += 200.0 * dt; 
        if self.note_y > 600.0 { // reset if off screen
            self.note_y = 0.0;
        }

        // TODO: Update systems here
        // self.audio_manager.update(dt);
        // let song_time = self.audio_manager.current_time();
        // self.note_spawner.update(dt, song_time);
        // self.input_manager.update();
        // self.score_manager.update(&self.note_spawner, &self.input_manager);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]), // background color
        );

        // Example: draw a rectangle as a placeholder "note"
        let rect = graphics::Rect::new(350.0, self.note_y, 100.0, 20.0);
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color::WHITE,
        )?;
        canvas.draw(&mesh, Vec2::new(0.0, 0.0));

        // TODO: Draw other systems
        // self.note_spawner.draw(ctx)?;
        // self.score_manager.draw(ctx)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Configure window (size + title)
    let cb = ggez::ContextBuilder::new("rhythm_game", "your_name")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust Rhythm Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
