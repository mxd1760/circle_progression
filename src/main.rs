/** Project outline
 *   - I want to make a game like agario.
 *      - the game has you collect a lot of little balls to increase your radius.
 *      - there are also other spheres controlled by other players (but i dont want to have this be multiplayer)
 *      - the bigger spheres alwasy eat the little spheres.
 *      - the bigger you are the slower you go
 *   - I've been inspired by palworld recently to so i want to add some ideas to the basics of agario
 *      - you can progress into different skills
 *   - the game needs some kind of challenge
 *      - i think enemies are the easiet thing to do
 *      - i think it would be interesting to have to empower the enemies as you power up to let players 
 *  Todos:
 *    - make little dots that you can collect
 *    - figure out what other leveling systems to add
 *    - figure out what the conflict/tention/challenge in the game is so the game is engaging rather than a chore.
 * 
 *  Done:
 *   - make sphere you can move around (maybe with some kind of visual reference in the background to track where you are)
 * 
 */

use eframe::egui;



struct Player{
  x:f32,
  y:f32,
  vx:f32,
  vy:f32,
  size: u32,
}
impl Default for Player{
  fn default() -> Self{
    Self{x:200.,y:200.,vx:0.,vy:0.,size:20}
  }
}

struct Settings{
  acceleration: f32,
  top_speed:f32,
  drag:f32,
}
impl Default for Settings{
  fn default() -> Self{
    Self{
      acceleration:0.2,
      top_speed:2500.,
      drag:0.03
    }
  }
}

struct World{
  left:f32,
  right:f32,
  top:f32,
  bottom:f32,
}
impl World{
  fn keep_in_bounds(&mut self, player: &mut Player){
    if player.x+player.size as f32 > self.right{
      player.x = self.right - player.size as f32;
      player.vx = -f32::abs(player.vx);
    }
    if player.y+player.size as f32 > self.bottom{
      player.y = self.bottom - player.size as f32;
      player.vy = -f32::abs(player.vy);
    }
    if self.top > player.y - player.size as f32 {
      player.y = self.top + player.size as f32;
      player.vy = f32::abs(player.vy);
    }
    if self.left > player.x - player.size as f32 {
      player.x = self.left + player.size as f32;
      player.vx = f32::abs(player.vx);
    }
  }
}

struct Game{
  player: Player,
  settings: Settings,
  world: World,
}

impl Game{
  fn handle_input(&mut self,input:&egui::InputState){
    let mut dir = (0.,0.);
    if input.key_down(egui::Key::W) || input.key_down(egui::Key::ArrowUp) {
      dir.1-=1.;
    }
    if input.key_down(egui::Key::S) || input.key_down(egui::Key::ArrowDown){
      dir.1+=1.;
    }
    if input.key_down(egui::Key::D) || input.key_down(egui::Key::ArrowRight){
      dir.0+=1.;
    }
    if input.key_down(egui::Key::A) || input.key_down(egui::Key::ArrowLeft){
      dir.0-=1.;
    }
    {
      let mag = f32::sqrt(dir.0*dir.0+dir.1*dir.1);
      if mag >1.{
        dir.1 /= mag;
        dir.0 /= mag;
      }
    }
    self.player.vx += dir.0 * self.settings.acceleration;
    self.player.vy += dir.1 * self.settings.acceleration;

    self.world.keep_in_bounds(&mut self.player);

    {
      let mag = f32::sqrt(self.player.vx*self.player.vx+self.player.vy*self.player.vy);
      if mag > self.settings.top_speed {
        self.player.vx *= self.settings.top_speed/mag;
        self.player.vy *= self.settings.top_speed/mag;
      }
    }
    self.player.x += self.player.vx;
    self.player.y += self.player.vy;

    self.player.vx *= 1.-self.settings.drag;
    self.player.vy *= 1.-self.settings.drag;

  }
}

impl Default for Game{
  fn default() -> Self{
    Self{
      player:Player::default(),
      settings:Settings::default(),
      world: World{top:0.,bottom:800.,left:0.,right:800.}
    }
  }
}
impl eframe::App for Game{
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
    ctx.input(|input| {
      self.handle_input(input);
    });
    egui::CentralPanel::default().show(ctx, |ui| {
      let painter = ui.painter();
      painter.circle(
        egui::Pos2{
          x: self.player.x,
          y: self.player.y,
        },
        self.player.size as f32,
        egui::Color32::from_rgb(100,150,200),
        egui::Stroke {
          width: 2.0,
          color: egui::Color32::from_rgb(255,255,255)
        }
      )
    });
    ctx.request_repaint();
  }
}

fn main() -> eframe::Result<()> {
  let native_options = eframe::NativeOptions{
    viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 800.0)),
    ..eframe::NativeOptions::default()
  };

  eframe::run_native(
    "Circle Progression",
    native_options,
    Box::new(|_ctx| Box::<Game>::default())
  )
}
