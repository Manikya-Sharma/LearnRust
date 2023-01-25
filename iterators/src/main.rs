fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }
    // assert_eq!(v1_iter.next(), Some(&1)); // needs mutable iter !
    // e.g. sum function takes ownership due to use of an iterator.

    // * Iterator adapters can be used to prevent loss of ownership.

    let v2: Vec<i32> = vec![1, 2, 3];
    // v2.iter().map(|x| x+1); // Wont do anything because iterators are lazy.
    // Closure never gets called because iterator never got consumed.
    // For these collect method is used which consumes the iterator.
    let v3: Vec<_> = v2.iter().map(|x| x + 5).collect();
    // assert_eq!(v3, vec![2, 3, 4])

    /*
    pub trait Iterator{
        type Item;

        fn next(&mut self) > Option<Self::Item>;
    }

    */
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ]
        );
    }
}
