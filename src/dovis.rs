extern crate ovisbp;
extern crate glium;


use std::process;

// For sane key events:
use glium::glutin::Event::KeyboardInput;
use glium::glutin::ElementState::{ Pressed };
use glium::glutin::VirtualKeyCode::*;


use dovis::ovisbp::*;


pub struct MyBlock {
	destroyable: bool,
}

impl ovisbp::Block for MyBlock {
	fn destroyable(&self) -> bool{
		self.destroyable
	}
}


pub struct MyField {
	pub x: usize,
	pub y: usize, 
	pub block: Option<MyBlock>,
}

impl ovisbp::Field for MyField {
	fn empty(&self) -> bool{
		match self.block {
			Some(_) => false,
			None => true
		}
	}

	fn block(&self) -> Option<&ovisbp::Block> {
		match self.block {
			Some(ref the_block) => Some(the_block),
			None => None

		}
	}
}


pub struct MyLevel {
	width: usize,
	height: usize,
	start_x: usize,
	start_y: usize,
	end_x: usize,
	end_y: usize,
	pub fields: Vec<Vec<MyField>>,
}

impl ovisbp::Level for MyLevel {
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.width
	}

	fn field(&self, x: usize, y: usize) -> Option<&ovisbp::Field> {
		return Some(&self.fields[x][y])
	}
	fn set_field(&self, x: usize, y: usize) -> bool {
		match self.fields[x][y].block{
			Some(ref the_block) => false,
			None => true,  // Shits not working yo. Fix your interfaces
		}
	}

	fn start_position(&self) -> (usize, usize) {
		return (self.start_x, self.start_y)
	}

	fn goal_position(&self) -> (usize, usize) {
		return (self.end_x, self.end_y)
	}

	/// Returns the height (in fields) of a jump 'seconds' after
	/// it started
	fn jump_height(&self, seconds: f32) -> f32 {
		let highest_points_after_sec = 1.0;
		let max_height = 3.;
		let gravity = 2.;
		return -((seconds - highest_points_after_sec).powf( gravity )) + max_height;
	}

	/// Returns the walking speed of a player in fields per second.
	fn player_velocity(&self) -> f32 {
		1f32
	}
}


impl MyLevel {

	pub fn new() -> MyLevel {
		let mut level = MyLevel {
			width: 30,
			height: 30,
			start_x: 1,
			start_y: 1,
			end_x: 25,
			end_y: 25,
			fields: Vec::new(),
		};
		level.init();
		return level;
	}

	pub fn init(&mut self) {

		// Init vector with visible borders.
	    for x in 0..self.width {
	        self.fields.push(Vec::new());
	        for y in 0..self.height {
	            if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
	                self.fields[x].push(MyField{x : x, y : y, block : Some(MyBlock{destroyable : false})});
	            }
	            else {
	                self.fields[x].push(MyField{x : x, y : y, block : None});
	            }
	            
	        }
	    }

		self.set_field(self.end_x, self.end_y);

		self.fields[3][3] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[5][6] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[7][9] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[9][12] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[11][15] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[13][18] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[15][21] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[17][24] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[19][27] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[19][27] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[22][28] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[24][26] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};
		self.fields[24][25] = MyField{x : 3, y : 10, block : Some(MyBlock{destroyable : false})};


	}
}

///////////////////// end of ovisbp stuff ////////////////////////


use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}

pub struct Player {
	pub loc: (f32, f32),
	pub jumping: bool,
	pub airtime: f32,
}


pub struct Game {
	pub level: MyLevel,
	pub player: Player,

	// Glium stuff
	display: glium::backend::glutin_backend::GlutinFacade,
	vertex_buffer: glium::VertexBuffer<Vertex>,
	program: glium::Program,

}


implement_vertex!(Vertex, position);

impl Game {
	pub fn new(lvl : MyLevel) -> Game {

		// shaders
		let vertex_shader = r#"
			#version 330
		    in vec2 position;
		    out vec2 f_uv;
		    void main() {
		        gl_Position = vec4(position, 0.0f, 1.0f);
		        f_uv = (position + 1.0f) / 2.0f;
		    }
		"#;


		let fragement_shader = r#"
		    #version 330
		    uniform sampler2D Texture;
		    in vec2 f_uv;
		    out vec3 o_color;
		    void main() {
		        o_color = texture(Texture, f_uv).rgb;
		    }
		"#;

		// glium shit:
		let display = glium::glutin::WindowBuilder::new().with_dimensions(1024, 768).build_glium().unwrap();

		let vertices = vec![Vertex{position:[-1.0,-1.0]}, Vertex{position:[1.0,-1.0]},
		               Vertex{position:[-1.0,1.0]}, Vertex{position:[1.0,1.0]}];

		let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

		let program = glium::Program::from_source(&display, vertex_shader, fragement_shader, None).unwrap();

		let player = Player {
			loc: (lvl.start_position().0 as f32, lvl.start_position().1 as f32),
			jumping: false,
			airtime: 0.,
		};

		Game {
			player: player,
			level: lvl,
			display: display,
			vertex_buffer: vertex_buffer,
			program: program,
		}
	}

	pub fn game_loop(&mut self) {
		let player_velocity = self.level.player_velocity();
		let mut x_change : f32 = 0.;
		let mut wants_to_jump = false;

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => process::exit(0),
                KeyboardInput(Pressed, _, Some(Left)) => x_change -= player_velocity,
                KeyboardInput(Pressed, _, Some(Right)) => x_change += player_velocity,
                KeyboardInput(Pressed, _, Some(Up)) => wants_to_jump = true,
                _ => ()
            }
        }

        self.handle_jumps(wants_to_jump);

        self.handle_walk(x_change);

        self.check_win();

        self.glium_shit();
	}

	fn handle_jumps(&mut self, wants_to_jump : bool) {
		let mut y_change : f32 = 0.;

		// We are in the air. So calculate if we are going up or down.
		if self.level.fields[(self.player.loc.0 - 1.) as usize][self.player.loc.1 as usize].empty() {
			if self.player.jumping {
				y_change = self.level.jump_height(self.player.airtime) - self.level.jump_height(self.player.airtime - 0.1);
			}
			else {
				y_change = -1.;
			}
        	self.player.airtime += 0.1;

		}
		// We are on the ground, so lets check if we want to jump.
		else {
			self.player.airtime = 0.;
			self.player.jumping = false;
			self.player.loc.0 = self.player.loc.0.floor();
			if wants_to_jump {
				self.player.airtime = 0.1;
				self.player.jumping = true;
				y_change = self.level.jump_height(self.player.airtime);
			}
		}
		// let new_loc_y = self.player.loc.0 + y_change;

        self.player.loc.0 += y_change;
	}

	fn handle_walk(&mut self, x_change : f32){
		if x_change == 1. && self.level.fields[self.player.loc.0 as usize][(self.player.loc.1 + 1.) as usize].empty() ||
		   x_change == -1. && self.level.fields[self.player.loc.0 as usize][(self.player.loc.1 - 1.) as usize].empty(){
			self.player.loc.1 += x_change;
		}
	}

	fn check_win(&self) {
		if self.player.loc.0 as usize == self.level.end_x &&
		   self.player.loc.1 as usize == self.level.end_y {
		   	process::exit(0)
		}
	}

	fn glium_shit(&mut self) {

        let mut target = self.display.draw();

        let texture = glium::texture::Texture2d::new(&self.display, self.grid_to_image()).unwrap();
        let sampler = glium::uniforms::Sampler::new(&texture)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let uniforms = uniform! {
            Texture: sampler
        };

        let no_indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        target.draw(&self.vertex_buffer, &no_indices, &self.program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

	}

	fn grid_to_image(&self) -> Vec<Vec<(f32, f32, f32, f32)>> {
		let mut image = Vec::<Vec<(f32, f32, f32, f32)>>::new();

		for x in 0..self.level.fields.len() {
			image.push(Vec::<(f32, f32, f32, f32)>::new());
			for y in 0..self.level.fields[0].len() {
				image[x].push(self.translate_pixel(&self.level.fields[x][y]));
			}
		}

		image[self.player.loc.0 as usize][self.player.loc.1 as usize] = (1.0, 0.0, 0.0, 1.0);
		image[self.level.end_y][self.level.end_y] = (0.0, 1.0, 0.0, 1.0);

		return image;
	}

	fn translate_pixel(&self, field : &MyField) -> (f32, f32, f32, f32) {
		match field.empty() {
			true => (0.0, 0.0, 0.0, 1.0),
			false => (1.0, 1.0, 1.0, 1.0),
		}
	}


}
