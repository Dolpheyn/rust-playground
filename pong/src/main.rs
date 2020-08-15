use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH: f32 = 640.00;
const WINDOW_HEIGHT: f32 = 480.00;

const PADDLE_SPEED: f32 = 8.0;

fn main() -> tetra::Result {
    ContextBuilder::new(
            "Pong", 
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32
        )
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Player {
    texture: Texture,
    position: Vec2<f32>,
}

impl Player {
    fn new(texture: Texture, position: Vec2<f32>) -> Self {
        Self { texture, position }
    }
}

struct GameState {
    players: Vec<Player>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut players: Vec<Player> = Vec::new();
        let texture = Texture::new(
            ctx, 
            "./resources/player1.png"
        )?;
        let position = Vec2::new(
            16.0, 
            (WINDOW_HEIGHT - texture.height() as f32) / 2.0
        );
        let player1 = Player::new(
            texture,
            position,
        );

        let texture = Texture::new(
            ctx, 
            "./resources/player2.png"
        )?;
        let position = Vec2::new(
            WINDOW_WIDTH - texture.width() as f32 - 16.00, 
            (WINDOW_HEIGHT - texture.height() as f32) / 2.0
        );
        let player2 = Player::new(
            texture,
            position,
        );
        
        players.push(player1);
        players.push(player2);

        let texture = Texture::new(ctx, "./resources/ball.png")?;
        let position = Vec2::new(
            WINDOW_WIDTH / 2.0 - texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - texture.height() as f32 / 2.0,
        );

        Ok(Self { players })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.players[0].texture, self.players[0].position);
        graphics::draw(ctx, &self.players[1].texture, self.players[1].position);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            if 
                (
                    self.players[0].position[1] -
                    PADDLE_SPEED 
                ) >= 0.0
                { self.players[0].position[1] -= PADDLE_SPEED; };
        };

        if input::is_key_down(ctx, Key::S) {
            if 
                (
                    self.players[0].position[1] + 
                    self.players[0].texture.height() as f32 + 
                    PADDLE_SPEED 
                ) < WINDOW_HEIGHT 
                { self.players[0].position[1] += PADDLE_SPEED; };
        };

        if input::is_key_down(ctx, Key::Up) {
            if 
                (
                    self.players[1].position[1] -
                    PADDLE_SPEED 
                ) >= 0.0
                { self.players[1].position[1] -= PADDLE_SPEED; };
        };

        if input::is_key_down(ctx, Key::Down) {
            if 
                (
                    self.players[1].position[1] + 
                    self.players[1].texture.height() as f32 + 
                    PADDLE_SPEED 
                ) < WINDOW_HEIGHT 
                { self.players[1].position[1] += PADDLE_SPEED; };
        };

        Ok(())
    }
}
