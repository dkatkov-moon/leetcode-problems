pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut max_area = 0.0;
    if points.len() >= 3 {
        let points_length = points.len();
        for i in 0..points_length {
            for j in 1..points_length {
                if i != j {
                    for k in 2..points_length {
                        if j != k {
                            let comp_area = area(&points[i], &points[j], &points[k]);
                            if comp_area > max_area {
                                max_area = comp_area;
                            }
                        }
                    }
                }
            }
        }
    }
    max_area
}

pub fn area(first: &Vec<i32>, second: &Vec<i32>, third: &Vec<i32>) -> f64 {
    return 0.5
        * (first[0] * (second[1] - third[1])
            + second[0] * (third[1] - first[1])
            + third[0] * (first[1] - second[1]))
            .abs() as f64;
}

fn main() {
    assert_eq!(
        largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );

    assert_eq!(
        largest_triangle_area(vec![vec![0, 0], vec![0, 3], vec![4, 0]]),
        6.0
    );
}
