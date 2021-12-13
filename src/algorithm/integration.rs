use ndarray::{ArrayView1, Array1, Zip};

const C: [f64; 7] = [
    0., 1./5., 3./10., 4./5., 8./9., 1., 1.
];

const A: [[f64; 6]; 7] = [
    [0.; 6],
    [1./5., 0., 0., 0., 0., 0.],
    [3./40., 9./40., 0., 0., 0., 0.],
    [44./45., -56./15., 32./9., 0., 0., 0.],
    [19372./6561., -25360./2187., 64448./6561., -212./729., 0., 0.],
    [9017./3168., -355./33., 46732./5247., 49./176., -5103./18656., 0.],
    [35./384., 0., 500./1113., 125./192., -2187./6784., 11./84.]
];

const B: [[f64; 7]; 2] = [
    [35./384., 0., 500./1113., 125./192., -2187./6784., 11./84., 0.],
    [5179./57600., 0., 7571./16695., 393./640., -92097./339200., 187./2100., 1./40.]
];

const E: [f64; 7] = [
    B[0][0] - B[1][0],
    B[0][1] - B[1][1],
    B[0][2] - B[1][2],
    B[0][3] - B[1][3],
    B[0][4] - B[1][4],
    B[0][5] - B[1][5],
    B[0][6] - B[1][6]
];

fn step(fcn: &impl Fn(ArrayView1<f64>, f64) -> Array1<f64>, y0: ArrayView1<f64>, f_k: &mut [Array1<f64>; 7], t0: f64, h: f64) {
    for k in 0..7 {
        let mut y_k: Array1<f64> = y0.to_owned();
        let t_k = t0 + C[k] * h;

        for i in 0..k {
            let a_loc = A[k][i];
            if a_loc != 0. {
                y_k += &(&f_k[i] * (h * a_loc));
            }
        }

        f_k[k] = fcn(y_k.view(), t_k);
    }
}

fn calculate_error(y0: ArrayView1<f64>, y_new: &mut Array1<f64>, f_k: &[Array1<f64>; 7], h: f64, threshold: f64) -> f64 {
    let mut f_e = E[0] * &f_k[0];
    for i in 2..7 {
        f_e += &(E[i] * &f_k[i]);
    }

    *y_new = &y0 + &((B[0][0]*&f_k[0] + B[0][2]*&f_k[2] + B[0][3]*&f_k[3] + B[0][4]*&f_k[4] + B[0][5]*&f_k[5]) * h);

    let error = Zip::from(&f_e)
        .and(&*y_new)
        .and(&y0)
        .map_collect(|&e, &a, &b| e.abs() / threshold.max(a.abs().max(b.abs())))
        .fold(0., |acc: f64, x| acc.max(*x)) * h;

    error
}

fn update_step(y_i: &mut Array1<f64>, t_i: &mut f64, f_k: &[Array1<f64>; 7], h: f64) {
    *y_i += &((B[0][0]*&f_k[0] + B[0][2]*&f_k[2] + B[0][3]*&f_k[3] + B[0][4]*&f_k[4] + B[0][5]*&f_k[5]) * h);
    *t_i += h;
}

pub fn dp45(fcn: impl Fn(ArrayView1<f64>, f64) -> Array1<f64>, y0: ArrayView1<f64>, tspan: [f64; 2], rtol: f64, atol: f64, h0: Option<f64>) -> Array1<f64> {
    let threshold = atol / rtol;
    let mut remaining = tspan[1] - tspan[0];

    // Calculate initial timestep if no one is provided
    let mut h = match h0 {
        Some(step) => step,
        None => {
            let mut step = (tspan[1] - tspan[0]) / 10.;
            let f0 = fcn(y0.view(), tspan[0]);
            let rh = Zip::from(&f0)
                .and(&y0)
                .map_collect(|&a, &b| a.abs() / threshold.max(b.abs()))
                .fold(0., |res: f64, x| res.max(*x)) / (0.8 * rtol.powf(0.2));
            
            if step * rh > 1. {
                step = 1. / rh;
            }

            step
        }
    };

    let mut y_new = Array1::<f64>::zeros(y0.len());

    // Initialize x_i and t_i variables.
    let mut y_i = y0.to_owned();
    let mut t_i = tspan[0];

    // Declare the vector which holds the intermediate solutions of the function provided at each timestep.
    let mut f_k: [Array1<f64>; 7] = Default::default();

    while remaining > 0. {
        if 1.1 * h > remaining {
            h = remaining;
            step(&fcn, y_i.view(), &mut f_k, t_i, h);

            update_step(&mut y_i, &mut t_i, &f_k, h);
            break;
        }

        step(&fcn, y_i.view(), &mut f_k, t_i, h);
        let error = calculate_error(y_i.view(), &mut y_new, &f_k, h, threshold);

        if error > rtol {
            h *= (0.8 * (rtol / error).powf(0.2)).max(0.1);
        } else {
            y_i.assign(&y_new);
            t_i += h;
            remaining -= h;

            let tmp = 1.25 * (error / rtol).powf(0.2);
            h /= tmp.max(0.2);
        }
    }

    y_i
}