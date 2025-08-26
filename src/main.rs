use ggez::event;
use ggez::graphics::{ self, Color };
use ggez::{ Context, GameResult };
use ggez::glam::*;

struct GameState {
    // TODO: Add your systems here
    // audio_manager: AudioManager,
    // note_spawner: NoteSpawner,
    // input_manager: InputManager,
    // score_manager: ScoreManager,
    note_y: f32,
    hit_line_y: f32,
    score: i32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let (_w, h) = ctx.gfx.drawable_size(); 
        
        Ok(GameState {
            note_y: 0.0,
            hit_line_y: h * 0.8,
            score: 0,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        self.note_y += 200.0 * dt;
        if self.note_y > self.hit_line_y + 100.0 {
            self.note_y = 0.0;
            self.score -= 10;
        }
        // Update systems here
        // self.audio_manager.update(dt);
        // let song_time = self.audio_manager.current_time();
        // self.note_spawner.update(dt, song_time);
        // self.input_manager.update();
        // self.score_manager.update(&self.note_spawner, &self.input_manager);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (win_w, _win_h) = ctx.gfx.drawable_size();
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]), // background color
        );

        let note_rect = graphics::Rect::new(380.0, self.note_y, 40.0, 20.0);
        let note_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            note_rect,
            Color::WHITE
        )?;
        canvas.draw(&note_mesh, Vec2::ZERO);

        let mesh = graphics::Mesh::new_line(
            ctx,
            &[Vec2::new(0.0, self.hit_line_y), Vec2::new(win_w, self.hit_line_y)],
            10.0,
            Color::WHITE,
        )?;
        canvas.draw(&mesh, Vec2::ZERO);
        // self.note_spawner.draw(&mut canvas, ctx)?;
        // self.score_manager.draw(ctx)?;

        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        canvas.draw(&score_text, Vec2::new(10.0, 10.0));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> GameResult {
        
        if let Some(keycode) = input.keycode {
            if keycode == ggez::input::keyboard::KeyCode::Space {
                let diff = (self.note_y - self.hit_line_y).abs();
                if diff < 10.0 {
                    self.score += 100;
                } else if diff < 30.0 {
                    self.score += 50;
                } else {
                    self.score -= 10;
                }
                self.note_y = 0.0;
            }
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    // let song_path = "C:\\dev\\yt-downloader\\Downloads\\Arknights ： IS#5 Bingo Lockout OST - One Last Gambit.mp3";

    // Configure window (size + title)
    let cb = ggez::ContextBuilder::new("rhythm_game", "your_name")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust Rhythm Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
