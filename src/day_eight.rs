use std::fs;
use itertools::Itertools;

fn get_input() -> String {
    fs::read_to_string("input/day_eight.txt")
        .expect("Something went wrong reading the file")
}

const WIDTH: i32 = 25;
const HEIGHT: i32 = 6;

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;


pub fn one() {
    let input_text = get_input();
    let layers = input_text
        .chars()
        .filter_map(|letter| letter.to_digit(10))
        .chunks((WIDTH * HEIGHT) as usize);
    let mut counter = vec![];
    for layer in &layers {
        let layer = layer.collect::<Vec<u32>>();
        let zero_count = layer.iter().filter(|digit| **digit == 0).count();
        let one_count = layer.iter().filter(|digit| **digit == 1).count();
        let two_count = layer.iter().filter(|digit| **digit == 2).count();

        counter.push((zero_count, one_count * two_count));
    }

    let min = counter.iter().min_by_key(|count| count.0);
    println!("{:?}", min);
}

pub fn two() {
    let input_text = get_input();
    let layer_data = input_text
        .chars()
        .filter_map(|letter| letter.to_digit(10))
        .chunks((WIDTH * HEIGHT) as usize);
    let mut layers = vec![];
    for layer in &layer_data {
        let layer = layer.collect::<Vec<u32>>();
        layers.push(Layer::new(layer))
    }

    // first layer is in the front
    layers.reverse();

    let mut last_layer = layers.first().unwrap().clone();
    for layer in layers.iter().skip(1) {
        last_layer.apply_layer(layer);
    }

    let display_char = if rand::random() {
        "🌈"
    } else {
        "❤️"
    };

    for line in last_layer.image_data.chunks(WIDTH as usize) {
        for pixel in line {
            print!("{}", if *pixel == 1 {display_char} else {" "});
        }
        println!("");
    }
}

#[derive(Clone, Debug)]
struct Layer {
    image_data: Vec<u32>,
}

impl Layer {
    fn new(image_data: Vec<u32>) -> Layer {
        Layer { image_data }
    }

    fn apply_layer(&mut self, other: &Layer) {
        for (i, pixel) in other.image_data.iter().enumerate() {
            if *pixel != TRANSPARENT {
                self.image_data[i] = *pixel;
            }
        }
    }
}
