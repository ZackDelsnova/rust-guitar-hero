use ggez::event;
use ggez::graphics::{ self, Color, Drawable };
use ggez::{ Context, GameResult };
use ggez::glam::*;
use rodio::{ Decoder, OutputStream, Sink };
use std::fs::File;

/// Your main game state
struct GameState {
    // Example variable for note position (falls down screen)
    note_y: f32,

    // TODO: Add your systems here
    audio_manager: AudioManager,
    note_spawner: NoteSpawner,
    // input_manager: InputManager,
    // score_manager: ScoreManager,
}

impl GameState {
    fn new(song_path: &str) -> GameResult<GameState> {
        let audio_manager = AudioManager::new(song_path);
        let note_spawner = NoteSpawner {
            notes_to_spawn: vec![],
            active_notes: vec![],
        };
        Ok(GameState {
            note_y: 0.0,
            audio_manager,
            note_spawner
        })
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

struct AudioManager {
    sink: Sink,
    elapsed: f32,
}

impl AudioManager {
    fn new(path: &str) -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        let file = File::open(path).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.play();
        AudioManager { sink, elapsed: 0.0 }
    }

    fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    fn current_time(&self) -> f32 {
        self.elapsed
    }

    fn is_finished(&self) -> bool {
        self.sink.empty()
    }
}

#[derive(Clone)]
struct Note {
    key: char,
    spawn_time: f32,
    y: f32,
}

struct NoteSpawner {
    notes_to_spawn: Vec<Note>,
    active_notes: Vec<Note>,
}

impl NoteSpawner {
    fn update(&mut self, dt: f32, song_time: f32) {
        // spawn note when timer approaches
        for note in &self.notes_to_spawn {
            if note.spawn_time <= song_time + 2.0 {
                self.active_notes.push(note.clone());
            }
        }

        for note in &mut self.active_notes {
            note.y += 200.0 * dt;
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) -> GameResult {
        for note in &self.active_notes {
            let rectangle = ggez::graphics::Rect::new(100.0, note.y, 50.0, 20.0);
            let mesh = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                rectangle,
                ggez::graphics::Color::WHITE,
            )?;
            canvas.draw(&mesh, Vec2::new(0.0, 0.0));
        }
        Ok(())
    }

}

pub fn main() -> GameResult {
    let song_path = "C:\\dev\\yt-downloader\\Downloads\\Arknights ： IS#5 Bingo Lockout OST - One Last Gambit.mp3";

    // Configure window (size + title)
    let cb = ggez::ContextBuilder::new("rhythm_game", "your_name")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust Rhythm Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new(song_path)?;
    event::run(ctx, event_loop, state)
}
