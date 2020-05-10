use clipboard::{ClipboardProvider, ClipboardContext};
use minifb::{Key, Window, WindowOptions, Scale};
use qrcode::QrCode;
use std::thread;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
	// Pull data from the clipboard.
	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
	let data:String = ctx.get_contents().unwrap();
	// ctx.set_contents()

	// Make a QR Code.
	let code = QrCode::new(data.as_bytes()).unwrap();
	//image = code.render::<Luma<u8>>().build()
	//image.save()
	//let qr_string = code.render::<char>().quiet_zone(false).module_dimensions(1, 1).build();
	let qr_string = code.render::<char>().quiet_zone(true).module_dimensions(1, 1).light_color('1').dark_color('0').build();
	let qr_strings:Vec<&str> = qr_string.split_whitespace().collect();
	//println!("{}", qr_string);

	// HACK: Reset width and height to zero so that we populate the window on first call.
	let mut width = WIDTH;
	let mut height = HEIGHT;
	let mut framebuffer: Vec<u32> = Vec::<u32>::with_capacity(width*height);
	width = 0;
	height = 0;

	// Pop up a window and display the image until someone hits escape.
	let mut window_options = WindowOptions::default();
	window_options.borderless = true;
	window_options.title = false;
	window_options.resize = false; // TODO: Figure out why the resizing glitch happens so we can make this resizable.
	window_options.scale = Scale::X1;
	let mut window = Window::new("", WIDTH, HEIGHT, window_options).unwrap_or_else(|e| {
		panic!("{}", e);
	});
	while window.is_open() && !window.is_key_down(Key::Escape) {
		{
			let new_size = window.get_size();
			if new_size.0 != width || new_size.1 != height {
				let pixel_height = new_size.1 / qr_strings.len();
				let pixel_width = new_size.0 / qr_strings[0].len();
				//println!("New size: {} {}  Old size: {} {}.", new_size.0, new_size.1, width, height);
				//println!("Pixel width / height: {} {}", pixel_width, pixel_height);
				width = new_size.0;
				height = new_size.1;
				framebuffer.resize(width*height, 0);
				for (y, row_data) in qr_strings.iter().enumerate() {
					for (x, column_data) in row_data.chars().into_iter().enumerate() {
						for dy in 0..=pixel_height {
							for dx in 0..=pixel_width {
								framebuffer[(x*pixel_width + dx)+(y*pixel_height + dy)*width] = 
									if column_data == '1' {
										0xFFFFFFFFu32
									} else {
										0x0
									}
							}
						}
					}
				}
			}
		}

		window.update_with_buffer(&framebuffer).unwrap();
		thread::sleep_ms(1); // yield_now() uses 100% cpu.  Odd.
	}
}
