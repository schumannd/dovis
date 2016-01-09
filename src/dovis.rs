extern crate ovisbp;
extern crate glium;


use std::process;


pub struct MyBlock {
	destroyable: bool,
}

impl ovisbp::Block for MyBlock {
	fn destroyable(&self) -> bool{
		self.destroyable
	}
}


pub struct MyField {
	pub x: u32,
	pub y: u32,
	// block: MyBlock,
}

// impl ovisbp::Field for MyField {
// 	fn empty(&self) -> bool{
// 		match self.block {
// 			Some(_) => true,
// 			None => false

// 		}
// 	}

// 	fn block(&self) -> Option<&ovisbp::Block>{
// 		match self.block {
// 			Some(_) => Some(&self.block),
// 			None => None

// 		}
// 	}
// }


pub struct MyLevel {
	pub width: usize,
	pub height: usize,
	pub start_x: usize,
	pub start_y: usize,
	pub end_x: usize,
	pub end_y: usize,
	pub field: Vec<Vec<usize>>,
}

impl ovisbp::Level for MyLevel {
	fn width(&self) -> usize{
		self.width
	}
	fn height(&self) -> usize{
		self.width
	}

	fn field(&self, x: usize, y: usize) -> Option<&ovisbp::Field>{
		None // TODO
	}
	fn set_field(&self, x: usize, y: usize) -> bool{
		false // TODO
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
		0f32 // TODO
	}

	/// Returns the walking speed of a player in fields per second.
	fn player_velocity(&self) -> f32{
		0f32 // TODO
	}
}


impl MyLevel {
	pub fn init(&mut self){
	    for x in 0..self.width {
	        self.field.push(Vec::new());
	        for y in 0..self.height {
	            if x == 1 || y == 1 || x == self.width - 2 || y == self.height - 2 {
	                self.field[x].push(1);
	            }else{
	                self.field[x].push(0);
	            }
	            
	        }
	    }
	}
	pub fn grid_to_image(&self) -> Vec<Vec<(f32, f32, f32, f32)>> {
		let mut image = Vec::<Vec<(f32, f32, f32, f32)>>::new();

		for x in 1..self.field.len() - 1 {
			image.push(Vec::<(f32, f32, f32, f32)>::new());
			for y in 1..self.field[0].len() - 1 {
				let gray_scale = self.field[x][y] as f32;
				image[x - 1 as usize].push((gray_scale, gray_scale, gray_scale, 1.0));
			}
		}
		return image;
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
	display: glium::backend::glutin_backend::GlutinFacade,
	vertices: Vec<Vertex>,
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
			level: lvl,
			display: display,
			vertices: vertices,
			vertex_buffer: vertex_buffer,
			program: program,
		}
	}

	pub fn game_loop(&mut self) {
        // Do game logic:
        // TODO
        self.level.field[self.level.start_x][self.level.start_y] = 1;

        self.glium_shit();

        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => process::exit(0),
                _ => ()
            }
        }
	}

	fn glium_shit(&mut self){

        let mut target = self.display.draw();

        let texture = glium::texture::Texture2d::new(&self.display, self.level.grid_to_image()).unwrap();
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


}
