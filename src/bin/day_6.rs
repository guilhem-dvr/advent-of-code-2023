fn main() {
    let races = vec![
        Race::new(47, 400),
        Race::new(98, 1213),
        Race::new(66, 1011),
        Race::new(98, 1540),
    ];

    let result = races
        .iter()
        .map(Race::get_number_of_ways_to_beat_record)
        .product::<u64>();

    println!("The result for part 1 is: {}", result);

    let big_race = Race::new(47986698, 400121310111540);
    let result = big_race.get_number_of_ways_to_beat_record();

    println!("The result for part 2 is: {}", result);
}

struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, record_distance: u64) -> Self {
        Race {
            time,
            record_distance,
        }
    }

    fn get_number_of_ways_to_beat_record(&self) -> u64 {
        let reduced_det = (self.time.pow(2) as f64) / 4.0 - (self.record_distance + 1) as f64;
        if reduced_det < 0.0 {
            return 0;
        } else if reduced_det == 0.0 {
            return 1;
        }

        let t_d_max = (self.time as f64) / 2.0;
        let sqrt_reduced_det = reduced_det.sqrt();

        let t_min_th = t_d_max - sqrt_reduced_det;
        let t_max_th = t_d_max + sqrt_reduced_det;

        let t_min = t_min_th.ceil() as u64;
        let t_max = t_max_th.floor() as u64;

        t_max - t_min + 1
    }
}

/*
    vitesse = T
    tdc = temps max - T
    distance = vitesse * tdc
    distance = T * temps_max - T^2
    cource gagnée = distance - distance record - 1 > 0
                  = - T^2 + temps_max * T - distance record - 1 > 0

    T min, max sont les temps limites pour gagner la course.
    Ils sont obtenus en resolvant l'équation polynomyale ci-dessus, pour une course donnée.

    determinant = temps_max^2 - 4 * distance record

    et T min, max = tdc/2 +/- racine(determinant)/2

    ou

    T min, max = tdc/2 +/- racine(tdc^2/4 + distance record)

    ces temps limites peuvent ne pas être entiers. Ils doivent aussi être positifs.

    => T min' = arrondi inf(T min) + 1
       T max' = arrondi sup(T max) - 1
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_finds_the_number_of_ways() {
        let races = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        let result: Vec<u64> = races
            .iter()
            .map(|r| r.get_number_of_ways_to_beat_record())
            .collect();

        assert_eq!(result, vec![4, 8, 9])
    }
}
