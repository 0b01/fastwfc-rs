extern crate fastwfc;

fn main() {
    for i in 0..100 {
        let runner = fastwfc::Overlapping::new(true, true, 100, 100, 1, false, 2);
        let input = image::open("fastwfc-sys/fast-wfc/example/samples/Chess.png")
            .unwrap()
            .to_rgba();
        let output = runner.run(input, 100);
    }
    // println!("{:#?}", output);
    // output.unwrap().save("out.png").unwrap();
}
