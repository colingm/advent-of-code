use std::fs;
use std::cmp;

type Layer = Vec<i32>;

pub struct Picture {
  pub width: usize,
  pub height: usize,
  pub layers: Vec<Layer>
}

impl Picture {
  pub fn new(width: usize, height: usize, input: &String) -> Self {
    let layer_size = width * height;
    let mut layers: Vec<Layer> = Vec::new();
    let mut cur = input.as_str();
    while !cur.is_empty() {
      let (chunk, rest) = cur.split_at(cmp::min(layer_size, cur.len()));

      let layer: Vec<i32> = chunk.trim().chars()
        .map(|el| {
          el.to_string().parse().unwrap()
        })
        .collect();

      if layer.len() > 0 {
        layers.push(layer);
      }

      cur = rest;
    }

    Self {
      width,
      height,
      layers
    }
  }

  pub fn fewest_digit_layer(&self, digit: i32) -> &Layer {
    let mut fewest_layer = 0;
    let mut fewest_count: i32 = -1;

    for (index, layer) in self.layers.iter().enumerate() {
      let count = layer.iter().filter(|&x| *x == digit).count();
      if fewest_count == -1 || count < fewest_count as usize {
        fewest_count = count as i32;
        fewest_layer = index;
      }
    }

    self.layers.get(fewest_layer).unwrap()
  }

  pub fn render_image(&self) {
    let mut final_image: Layer = vec![2; self.width * self.height];

    for layer in &self.layers {
      for (index, pixel) in layer.iter().enumerate() {
        if final_image[index] == 2 {
          final_image[index] = *pixel;
        }
      }
    }

    for i in 0..self.height {
      let begin = i * self.width;
      let end = (i + 1) * self.width;
      let row: String = final_image[begin..end].iter().map(|e| if *e == 0 { "  " } else { "██" }).collect();
      println!("{:?}", row);
    }
  }
}

fn main() {
  let contents: String = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

  let picture = Picture::new(25, 6, &contents);
  let layer = picture.fewest_digit_layer(0);
  let ones = layer.iter().filter(|&x| *x == 1).count();
  let twos = layer.iter().filter(|&x| *x == 2).count();

  println!("Part 1: {}", ones * twos);
  println!("Part 2");
  picture.render_image();
}
