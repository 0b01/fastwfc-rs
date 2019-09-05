extern crate fastwfc;

fn main() {
    for i in 0..1000 {
        let runner = fastwfc::Overlapping::new(100, 100, 2);
        let input = image::open("fastwfc-sys/fast-wfc/example/samples/Chess.png")
            .unwrap()
            .to_rgba();
        let output = runner.generate(input, 100);
    }
    // println!("{:#?}", output);
    // output.unwrap().save("out.png").unwrap();
}
