pub struct SimpleColorMap {
    colors: Vec<[u8; 3]>,
    props: Vec<f64>,
}

impl SimpleColorMap {
    pub fn new(colors: Vec<[u8; 3]>, props: Vec<f64>) -> SimpleColorMap {
        SimpleColorMap { colors, props }
    }

    pub fn get_color(&self, prop: f64) -> [u8; 3] {
        let (index_a, index_b) = get_surounding_index(&self.props, prop);
        if index_a == index_b {
            return self.colors[index_a];
        }
        let color_a = self.colors[index_a];
        let color_b = self.colors[index_b];
        let prop = (prop - self.props[index_a]) / (self.props[index_b] - self.props[index_a]);
        lerp_color(color_a, color_b, prop.clamp(0.0, 1.0))
    }
}

fn lerp_color(c1: [u8; 3], c2: [u8; 3], prop: f64) -> [u8; 3] {
    [
        (c1[0] as f64 + (c2[0] as f64 - c1[0] as f64) * prop) as u8,
        (c1[1] as f64 + (c2[1] as f64 - c1[1] as f64) * prop) as u8,
        (c1[2] as f64 + (c2[2] as f64 - c1[2] as f64) * prop) as u8,
    ]
}

fn get_surounding_index(props: &[f64], target: f64) -> (usize, usize) {
    if target <= props[0] {
        (0, 0)
    } else if target >= props[props.len() - 1] {
        (props.len() - 1, props.len() - 1)
    } else {
        let mut index = 1;
        for (i, prop) in props.iter().enumerate() {
            if target < *prop {
                index = i;
                break;
            }
        }
        (index - 1, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color_exact_match() {
        let colormap = SimpleColorMap::new(
            vec![
                [50, 110, 150],
                [200, 200, 180],
                [100, 150, 70],
                [60, 90, 55],
                [210, 210, 210],
            ],
            vec![0.0, 0.01, 0.03, 0.35, 0.6],
        );

        assert_eq!(colormap.get_color(0.0), [50, 110, 150]);
        assert_eq!(colormap.get_color(0.01), [200, 200, 180]);
        assert_eq!(colormap.get_color(0.03), [100, 150, 70]);
        assert_eq!(colormap.get_color(0.35), [60, 90, 55]);
        assert_eq!(colormap.get_color(0.6), [210, 210, 210]);
    }

    #[test]
    fn test_get_color_interpolation() {
        let colormap = SimpleColorMap::new(
            vec![
                [50, 110, 150],
                [200, 200, 180],
                [100, 150, 70],
                [60, 90, 55],
                [210, 210, 210],
            ],
            vec![0.0, 0.01, 0.03, 0.35, 0.6],
        );

        assert_eq!(colormap.get_color(0.005), [125, 155, 165]);
        assert_eq!(colormap.get_color(0.02), [150, 175, 124]);
        assert_eq!(colormap.get_color(0.2), [78, 118, 62]);
        assert_eq!(colormap.get_color(0.5), [150, 162, 148]);
    }

    #[test]
    fn test_get_color_out_of_bounds() {
        let colormap = SimpleColorMap::new(
            vec![
                [50, 110, 150],
                [200, 200, 180],
                [100, 150, 70],
                [60, 90, 55],
                [210, 210, 210],
            ],
            vec![0.0, 0.01, 0.03, 0.35, 0.6],
        );

        assert_eq!(colormap.get_color(-0.1), [50, 110, 150]);
        assert_eq!(colormap.get_color(1.0), [210, 210, 210]);
    }
}
