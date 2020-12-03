use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct SpaceObject {
  name: String,
  orbiting_objects: Vec<String>
}

impl SpaceObject {
  pub fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      orbiting_objects: Vec::new()
    }
  }

  pub fn add_orbiting_object(&mut self, object: &str) {
    self.orbiting_objects.push(String::from(object));
  }

  pub fn total_orbits(&self, current_distance: usize, universe: &Universe) -> usize {
    if self.orbiting_objects.len() == 0 {
      current_distance
    } else {
      let mut orbit_count = current_distance;
      for space_object in self.orbiting_objects.iter() {
        orbit_count += universe.objects
          .get(space_object)
          .unwrap()
          .total_orbits(current_distance + 1, universe);
      }

      orbit_count
    }
  }

  pub fn find_object(&self, find: &String, universe: &Universe) -> Option<Vec<String>> {
    if &self.name == find {
      Some(vec![self.name.clone()])
    } else {
      for space_object in self.orbiting_objects.iter() {
        let object = universe.objects.get(space_object).unwrap();
        if let Some(mut found_objects) = object.find_object(find, universe) {
          found_objects.push(self.name.clone());
          return Some(found_objects);
        }
      }

      None
    }
  }
}

struct Universe {
  objects: HashMap<String, SpaceObject>
}

impl Universe {
  pub fn new() -> Self {
    Self {
      objects: HashMap::new()
    }
  }

  pub fn process_orbits(&mut self, orbits: &Vec<String>) {
    orbits.iter().for_each(|el| {
      let parts: Vec<&str> = el.trim().split(")").collect();
      let inner_name = parts[0];
      let outer_name = parts[1];

      if !self.objects.contains_key(&String::from(inner_name)) {
        self.objects.insert(String::from(inner_name), SpaceObject::new(inner_name));
      }
      if !self.objects.contains_key(&String::from(outer_name)) {
        self.objects.insert(String::from(outer_name), SpaceObject::new(outer_name));
      }

      let object = self.objects.get_mut(&String::from(inner_name)).unwrap();
      object.add_orbiting_object(outer_name);
    });
  }

  pub fn total_orbits(&self) -> usize {
    match &self.objects.get(&String::from("COM")) {
      Some(com) => com.total_orbits(0, self),
      None => {
        panic!("Universe is not created yet");
      }
    }
  }

  pub fn total_transfers(&self, o1: &str, o2: &str) -> usize {
    let (p1, p2) = (self.build_path(&String::from(o1)), self.build_path(&String::from(o2)));

    for (i, x) in p1.iter().enumerate() {
      for (j, y) in p2.iter().enumerate() {
        if x == y {
          return (i - 1) + (j - 1);
        }
      }
    }

    return 0;
  }

  fn build_path(&self, object: &String) -> Vec<String> {
    match &self.objects.get(&String::from("COM")) {
      Some(com) => {
        com.find_object(object, self).unwrap()
      },
      None => {
        panic!("Universe is not created yet");
      }
    }
  }
}

fn get_space_objects(filename: &str) -> Vec<String> {
  let contents = fs::read_to_string(filename)
      .expect("Something went wrong reading the file");
  let space_objects: Vec<String> = contents
    .lines()
    .map(ToOwned::to_owned)
    .collect();

  space_objects
}

fn main() {
  let space_objects = get_space_objects("input.txt");

  let mut universe = Universe::new();
  universe.process_orbits(&space_objects);

  println!("Total orbits in universe: {}", universe.total_orbits());
  println!("Total transfers to Santa: {}", universe.total_transfers("YOU", "SAN"));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_orbits() {
    let space_objects = get_space_objects("test.txt");

    let mut universe = Universe::new();
    universe.process_orbits(&space_objects);

    assert_eq!(universe.total_orbits(), 42);
  }

  #[test]
  fn test_paths() {
    let space_objects = get_space_objects("test2.txt");

    let mut universe = Universe::new();
    universe.process_orbits(&space_objects);

    assert_eq!(universe.total_transfers("YOU", "SAN"), 4);
  }
}
