#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

const STACK_WIDTH: i32 = 100;
const STACK_HEIGHT: i32 = 400;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { name: String::from("CPU stack"),
	    stack: Stack { sp: 0x8a00,
			   range: (0x8000, 0x8fff),
			   highmark: 0x8230,
			   reg: String::from("sp"),
	    }
    }
}

// ------ ------
//     Model
// ------ ------

type Address = u32;

struct Model {
    name: String,
    stack: Stack
}

struct Stack {
    range: (Address, Address),
    highmark: Address,
    sp: Address,
    reg: String,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.stack.sp += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let x = 2;
    let y = 2;
    let line_width = 1;
    let (start, end) = model.stack.range;
    let size = (end - start + 1) as f32;
    let overflow = model.stack.sp < start;
    let used = if overflow { 1.0 } else { (end - model.stack.sp) as f32 / size };
    let in_use = used * STACK_HEIGHT as f32;
    let past_used = if overflow { 0.0 } else { (model.stack.sp - model.stack.highmark) as f32 / size };
    let past_use = past_used * STACK_HEIGHT as f32;
    let stack_pointer = if overflow { y + STACK_HEIGHT } else { y + in_use as i32 };

    let (use_color, use_opacity) = if overflow { ("red", 0.75) } else  { ("black", 0.25) };
    let past_use_color = if used + past_used > 0.9 { "red" } else { "black" };

    let font_height = 12;

    svg![
	attrs! {
	    At::ViewBox => "0 0 800 800"
	},
	line_![
	    attrs! {
		At::Stroke => "black",
		At::StrokeWidth => line_width,
		At::X1 => x,
		At::Y1 => stack_pointer,
		At::X2 => STACK_WIDTH + 25,
		At::Y2 => stack_pointer,
	    }
	],
	text![
	    attrs! {
		At::X => x + STACK_WIDTH + 25,
		At::Y => stack_pointer - 1,
		At::FontSize => font_height,
	    },
	    &model.stack.reg
	],
	rect![
	    attrs! {
		At::Stroke => "black",
		At::StrokeWidth => line_width,
		At::StrokeOpacity => 0.8,
		At::Width => STACK_WIDTH,
		At::Height => STACK_HEIGHT,
		At::X => x,
		At::Y => y,
		At::Fill => "none",
	    }
	],
	rect![
	    attrs! {
		At::Fill => use_color
		At::FillOpacity => use_opacity,
		At::X => x,
		At::Y => y,
		At::Width => STACK_WIDTH,
		At::Height => in_use,
	    }
	],
	rect![
	    attrs! {
		At::Fill => past_use_color,
		At::FillOpacity => 0.15,
		At::X => x,
		At::Y => stack_pointer,
		At::Width => STACK_WIDTH,
		At::Height => past_use,
	    }
	],
	text![
	    attrs! {
		At::X => x + STACK_WIDTH / 2,
		At::Y => y + STACK_HEIGHT + font_height + 2;
		At::FontSize => font_height,
		At::TextAnchor => "middle",
	    },
	    &model.name
	]

    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
