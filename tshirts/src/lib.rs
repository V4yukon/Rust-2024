


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_prefer: Option<ShirtColor>) -> ShirtColor {
        user_prefer.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                &ShirtColor::Blue => num_blue +=1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_some() {
        let user_prefer: Option<ShirtColor> = Some(ShirtColor::Red);
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
        assert_eq!(ShirtColor::Blue, store.giveaway(user_prefer));
    }
}