use proconio::input;

const WATER_UNIT: usize = 100;

fn main() {
    input! {
        (water1, water2): (usize, usize),
        (sugar1, sugar2): (usize, usize),
        concentration_limit: usize,
        weight_limit: usize,
    }

    let boundary_sugared_water = SugaredWater {
        water_weight: 100,
        sugar_weight: concentration_limit,
    };

    let mut best_sugared_water: Option<SugaredWater> = None;

    for water_weight_1 in (0..=weight_limit).step_by(WATER_UNIT * water1) {
        for water_weight_2 in (0..=(weight_limit - water_weight_1)).step_by(WATER_UNIT * water2) {
            if water_weight_1 == 0 && water_weight_2 == 0 {
                continue;
            }

            let water_weight = water_weight_1 + water_weight_2;

            for sugar_weight_1 in (0..=(weight_limit - water_weight)).step_by(sugar1) {
                let base_weight = water_weight + sugar_weight_1;

                let base_sugared_water = SugaredWater {
                    water_weight,
                    sugar_weight: sugar_weight_1,
                };

                if base_sugared_water > boundary_sugared_water {
                    break;
                }

                let mut ok = 0_usize;
                let mut ng = (weight_limit - base_weight) / sugar2 + 1;

                while ok.abs_diff(ng) > 1 {
                    let mid = (ok + ng) / 2;
                    let sugared_water = base_sugared_water.added_sugar(sugar2 * mid);

                    if sugared_water <= boundary_sugared_water {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }

                let sugared_water = base_sugared_water.added_sugar(sugar2 * ok);
                chmax_for_option(&mut best_sugared_water, sugared_water);
            }
        }
    }

    let best_sugared_water = best_sugared_water.unwrap();
    println!(
        "{} {}",
        best_sugared_water.sum_weight(),
        best_sugared_water.sugar_weight
    );
}

struct SugaredWater {
    water_weight: usize,
    sugar_weight: usize,
}

impl SugaredWater {
    fn sum_weight(&self) -> usize {
        self.water_weight + self.sugar_weight
    }

    fn added_sugar(&self, add_sugar_weight: usize) -> Self {
        Self {
            water_weight: self.water_weight,
            sugar_weight: self.sugar_weight + add_sugar_weight,
        }
    }
}

impl PartialEq for SugaredWater {
    fn eq(&self, other: &Self) -> bool {
        self.sugar_weight * other.sum_weight() == other.sugar_weight * self.sum_weight()
    }
}

impl Eq for SugaredWater {}

impl PartialOrd for SugaredWater {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.sugar_weight * other.sum_weight())
            .partial_cmp(&(other.sugar_weight * self.sum_weight()))
    }
}

impl Ord for SugaredWater {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
