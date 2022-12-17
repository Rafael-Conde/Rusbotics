#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use Rusbotics::gui_functions::get_gui;

fn main()
{
    let gui = get_gui();
    gui.run_gui();
}
