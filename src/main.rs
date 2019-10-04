use clipboard::{ClipboardProvider, ClipboardContext};
use minifb::{Key, Window, WindowOptions};
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
	//let s = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();
	//println!("{}", s);
	let mut width = 0;
	let mut height = 0;
	let framebuffer: Vec<u32> = code.render::<char>().quiet_zone(true).module_dimensions(1, 1).light_color('1').dark_color('0').build().chars().map(|character|{
		match character {
			'1' => { width += 1; 0xFFFFFFFF as u32 },
			'0' => { width += 1; 0x0 as u32 },
			'\n' => { width += 1; height += 1; 0x0 as u32 }
			_ => { width += 1; 0x0 as u32 }
		}
	}).collect();

	println!("Width / Height: {} {}", width, height);

	// Pop up a window and display the image until someone hits escape.
	let mut window_options = WindowOptions::default();
	window_options.borderless = true;
	window_options.title = false;
	window_options.resize = true;
	let mut window = Window::new("", width, height, window_options).unwrap_or_else(|e| {
		panic!("{}", e);
	});
	while window.is_open() && !window.is_key_down(Key::Escape) {
		window.update_with_buffer(&framebuffer).unwrap();
		thread::sleep_ms(1); // yield_now() uses 100% cpu.  Odd.
	}
}
