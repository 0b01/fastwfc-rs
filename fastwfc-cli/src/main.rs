extern crate fastwfc;
extern crate structopt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wfcgen", about = "Use fastwfc to generate an image.")]
struct Opt {
    /// Toric input
    #[structopt(long)]
    pub periodic_input: bool,
    /// Toric output
    #[structopt(long)]
    pub periodic_output: bool,

    /// The height of the output in pixels
    #[structopt(short, long)]
    pub height: u64,
    /// The width of the output in pixels
    #[structopt(short, long)]
    pub width: u64,

    /// Number of symmetries from 0 to 8.
    /// If the pattern already exist, increase its number of appearance.
    #[structopt(long, default_value = "8")]
    pub symmetry: u8,
    /// Output contains ground
    #[structopt(short, long)]
    pub ground: bool,

    /// Width and height in pixel of the patterns
    #[structopt(short, long)]
    pub size: u64,

    /// Maximum number of trials
    #[structopt(short, long)]
    pub tries: u32,

    /// Input file
    #[structopt(short, parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(short, parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let overlapping = fastwfc::Overlapping {
        periodic_input: opt.periodic_output,
        periodic_output: opt.periodic_output,
        out_height: opt.height,
        out_width: opt.width,
        symmetry: opt.symmetry,
        ground: opt.ground,
        pattern_size: opt.size,
    };

    let input_img = image::open(opt.input).expect("Cannot open image");
    let output = overlapping.generate(input_img.to_rgba(), opt.tries).expect("Failed to generate");
    output.save(opt.output).expect("unable to save file");
}

// fn main() {
//     for _i in 0..1000 {
//         let runner = fastwfc::Overlapping::new(100, 100, 2);
//         let input = image::open("fastwfc-sys/fast-wfc/example/samples/Chess.png")
//             .unwrap()
//             .to_rgba();
//         let output = runner.generate(input, 100);
//     }
// }
