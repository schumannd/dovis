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
			Some(_) => true,
			None => false
		}
	}

	fn block(&self) -> Option<&ovisbp::Block>{
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
	fn width(&self) -> usize{
		self.width
	}
	fn height(&self) -> usize{
		self.width
	}

	fn field(&self, x: usize, y: usize) -> Option<&ovisbp::Field>{
		return Some(&self.fields[x][y])
	}
	fn set_field(&self, x: usize, y: usize) -> bool{
		match self.fields[x][y].block{
			Some(ref the_block) => false,
			None => true,  // Shits not working yo. Fix your interfaces
		}
	}

	fn start_position(&self) -> (usize, usize){
		return (self.start_x, self.start_y)
	}

	fn goal_position(&self) -> (usize, usize){
		return (self.end_x, self.end_y)
	}

	/// Returns the height (in fields) of a jump 'seconds' after
	/// it started
	fn jump_height(&self, seconds: f32) -> f32{
		if seconds <= 3f32{
			return 3f32 - seconds
		}
		return 0f32
	}

	/// Returns the walking speed of a player in fields per second.
	fn player_velocity(&self) -> f32{
		1f32
	}
}


impl MyLevel {

	pub fn new() -> MyLevel{
		let mut level = MyLevel{
			width: 100,
			height: 100,
			start_x: 3,
			start_y: 3,
			end_x: 95,
			end_y: 95,
			fields: Vec::new(),
		};
		level.init();
		return level;
	}

	pub fn init(&mut self){

		// Init vector with visible borders.
	    for x in 0..self.width {
	        self.fields.push(Vec::new());
	        for y in 0..self.height {
	            if x == 1 || y == 1 || x == self.width - 2 || y == self.height - 2 {
	                self.fields[x].push(MyField{x : x, y : y, block : Some(MyBlock{destroyable : false})});
	            }else{
	                self.fields[x].push(MyField{x : x, y : y, block : None});
	            }
	            
	        }
	    }

		self.set_field(self.end_x, self.end_y);


	}
}

///////////////////// end of ovisbp stuff ////////////////////////


use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}


pub struct Game{
	pub level: MyLevel,
	pub player: (f32, f32),

	// Glium stuff
	display: glium::backend::glutin_backend::GlutinFacade,
	vertex_buffer: glium::VertexBuffer<Vertex>,
	program: glium::Program,

}


implement_vertex!(Vertex, position);

impl Game {
	pub fn new(lvl : MyLevel) -> Game{

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
		Game{
			player: (lvl.start_position().0 as f32, lvl.start_position().1 as f32),
			level: lvl,
			display: display,
			vertex_buffer: vertex_buffer,
			program: program,
		}
	}

	pub fn game_loop(&mut self) {
		let mut x_change : f32 = 0.;
		let mut y_change : f32 = 0.;
		let player_velocity = self.level.player_velocity();

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => process::exit(0),
                KeyboardInput(Pressed, _, Some(Left)) => x_change -= player_velocity,
                KeyboardInput(Pressed, _, Some(Right)) => x_change += player_velocity,
                KeyboardInput(Pressed, _, Some(Up)) => y_change += player_velocity,
                KeyboardInput(Pressed, _, Some(Down)) => y_change -= player_velocity,
                _ => ()
            }
        }
        // reset old position
        // self.level.fields[self.level.player.0.floor() as usize][self.level.player.1.floor() as usize].block.destroyable = 0;

        self.player.0 += y_change;
        self.player.1 += x_change;

        // set new position
        // self.level.fields[self.level.player.0.floor() as usize][self.level.player.1.floor() as usize] = 1;

        self.glium_shit();
	}

	fn glium_shit(&mut self){

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

		for x in 1..self.level.fields.len() - 1 {
			image.push(Vec::<(f32, f32, f32, f32)>::new());
			for y in 1..self.level.fields[0].len() - 1 {
				image[x - 1 as usize].push(self.translate_pixel(&self.level.fields[x][y]));
			}
		}

		image[self.player.0 as usize][self.player.1 as usize] = (1.0, 0.0, 0.0, 1.0);

		return image;
	}

	fn translate_pixel(&self, field : &MyField) -> (f32, f32, f32, f32) {
		match field.empty() {
			true => (1.0, 1.0, 1.0, 1.0),
			false => (0.0, 0.0, 0.0, 1.0),
		}
	}


}
