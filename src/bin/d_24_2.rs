use nalgebra::{Matrix3, Matrix6, Vector3, Vector6};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_24_1.txt");

struct HailStone {
    pos: Vector3<i64>,
    velocity: Vector3<i64>,
}

fn main() {
    let hs = INPUT_FILE
        .lines()
        .map(|line| {
            let (coords, velocity) = line.split_once('@').unwrap();
            let mut coords = coords
                .split(',')
                .map(str::trim)
                .map(|v| v.parse::<i64>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            let z = coords.next().unwrap();
            let pos = Vector3::new(x, y, z);
            let mut velocity = velocity
                .split(',')
                .map(str::trim)
                .map(|v| v.parse::<i64>().unwrap());
            let vx = velocity.next().unwrap();
            let vy = velocity.next().unwrap();
            let vz = velocity.next().unwrap();
            let velocity = Vector3::new(vx, vy, vz);

            HailStone { pos, velocity }
        })
        .take(3)
        .collect::<Vec<_>>();

    let s1 = -hs[0].pos.cross(&hs[0].velocity) + hs[1].pos.cross(&hs[1].velocity);
    let s2 = -hs[0].pos.cross(&hs[0].velocity) + hs[2].pos.cross(&hs[2].velocity);
    let right = Vector6::new(
        s1.x as f64,
        s1.y as f64,
        s1.z as f64,
        s2.x as f64,
        s2.y as f64,
        s2.z as f64,
    );
    let mut solver = Matrix6::<f64>::identity();
    let ub = cross_matrix(hs[0].velocity) - cross_matrix(hs[1].velocity);
    solver[(0, 0)] = ub[(0, 0)];
    solver[(0, 1)] = ub[(0, 1)];
    solver[(0, 2)] = ub[(0, 2)];
    solver[(1, 0)] = ub[(1, 0)];
    solver[(1, 1)] = ub[(1, 1)];
    solver[(1, 2)] = ub[(1, 2)];
    solver[(2, 0)] = ub[(2, 0)];
    solver[(2, 1)] = ub[(2, 1)];
    solver[(2, 2)] = ub[(2, 2)];
    let ub = cross_matrix(hs[0].velocity) - cross_matrix(hs[2].velocity);
    solver[(3, 0)] = ub[(0, 0)];
    solver[(3, 1)] = ub[(0, 1)];
    solver[(3, 2)] = ub[(0, 2)];
    solver[(4, 0)] = ub[(1, 0)];
    solver[(4, 1)] = ub[(1, 1)];
    solver[(4, 2)] = ub[(1, 2)];
    solver[(5, 0)] = ub[(2, 0)];
    solver[(5, 1)] = ub[(2, 1)];
    solver[(5, 2)] = ub[(2, 2)];
    let ub = -cross_matrix(hs[0].pos) + cross_matrix(hs[1].pos);
    solver[(0, 3)] = ub[(0, 0)];
    solver[(0, 4)] = ub[(0, 1)];
    solver[(0, 5)] = ub[(0, 2)];
    solver[(1, 3)] = ub[(1, 0)];
    solver[(1, 4)] = ub[(1, 1)];
    solver[(1, 5)] = ub[(1, 2)];
    solver[(2, 3)] = ub[(2, 0)];
    solver[(2, 4)] = ub[(2, 1)];
    solver[(2, 5)] = ub[(2, 2)];
    let ub = -cross_matrix(hs[0].pos) + cross_matrix(hs[2].pos);
    solver[(3, 3)] = ub[(0, 0)];
    solver[(3, 4)] = ub[(0, 1)];
    solver[(3, 5)] = ub[(0, 2)];
    solver[(4, 3)] = ub[(1, 0)];
    solver[(4, 4)] = ub[(1, 1)];
    solver[(4, 5)] = ub[(1, 2)];
    solver[(5, 3)] = ub[(2, 0)];
    solver[(5, 4)] = ub[(2, 1)];
    solver[(5, 5)] = ub[(2, 2)];

    let r = solver.try_inverse().unwrap() * right;

    let pos_sum = r.x + r.y + r.z;
    println!("Start position coordinate sum is {pos_sum}");
}

fn cross_matrix(i: Vector3<i64>) -> Matrix3<f64> {
    Matrix3::new(
        0 as f64,
        -i[2] as f64,
        i[1] as f64,
        i[2] as f64,
        0 as f64,
        -i[0] as f64,
        -i[1] as f64,
        i[0] as f64,
        0 as f64,
    )
}
