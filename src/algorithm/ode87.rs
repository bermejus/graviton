use ndarray::{Array1, ArrayView1, Zip};

const C: [f64; 13] = [
    0., 1./18., 1./12., 1./8., 5./16., 3./8., 59./400., 93./200., 5490023248./9719169821., 13./20., 1201146811./1299019798., 1., 1.
];

const A: [[f64; 12]; 13] = [
    [0.; 12],
    [1./18., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.], 
    [1./48., 1./16., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
    [1./32., 0., 3./32., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
    [5./16., 0., -75./64., 75./64., 0., 0., 0., 0., 0., 0., 0., 0.],
    [3./80., 0., 0., 3./16., 3./20., 0., 0., 0., 0., 0., 0., 0.],
    [29443841./614563906., 0., 0., 77736538./692538347., -28693883./1125000000., 23124283./1800000000., 0., 0., 0., 0., 0., 0.],
    [16016141./946692911., 0., 0., 61564180./158732637., 22789713./633445777., 545815736./2771057229., -180193667./1043307555., 0., 0., 0., 0., 0.],
    [39632708./573591083., 0., 0., -433636366./683701615., -421739975./2616292301., 100302831./723423059., 790204164./839813087., 800635310./3783071287., 0., 0., 0., 0.],
    [246121993./1340847787., 0., 0., -37695042795./15268766246., -309121744./1061227803., -12992083./490766935., 6005943493./2108947869., 393006217./1396673457., 123872331./1001029789., 0., 0., 0.],
    [-1028468189./846180014., 0., 0., 8478235783./508512852., 1311729495./1432422823., -10304129995./1701304382., -48777925059./3047939560., 15336726248./1032824649., -45442868181./3398467696., 3065993473./597172653., 0., 0.],
    [185892177./718116043., 0., 0., -3185094517./667107341., -477755414./1098053517., -703635378./230739211., 5731566787./1027545527., 5232866602./850066563., -4093664535./808688257., 3962137247./1805957418., 65686358./487910083., 0.],
    [403863854./491063109., 0., 0., -5068492393./434740067., -411421997./543043805., 652783627./914296604., 11173962825./925320556., -13158990841./6184727034., 3936647629./1978049680., -160528059./685178525., 248638103./1413531060., 0.]
];

const B: [[f64; 13]; 2] = [
    [13451932./455176623., 0., 0., 0., 0., -808719846./976000145., 1757004468./5645159321., 656045339./265891186., -3867574721./1518517206., 465885868./322736535., 53011238./667516719., 2./45., 0.],
    [14005451./335480064., 0., 0., 0., 0., -59238493./1068277825., 181606767./758867731., 561292985./797845732., -1041891430./1371343529., 760417239./1151165299., 118820643./751138087., -528747749./2220607170., 1./4.]
];

const E: [f64; 13] = [
    B[0][0] - B[1][0],
    B[0][1] - B[1][1],
    B[0][2] - B[1][2],
    B[0][3] - B[1][3],
    B[0][4] - B[1][4],
    B[0][5] - B[1][5],
    B[0][6] - B[1][6],
    B[0][7] - B[1][7],
    B[0][8] - B[1][8],
    B[0][9] - B[1][9],
    B[0][10] - B[1][10],
    B[0][11] - B[1][11],
    B[0][12] - B[1][12]
];

fn step(fcn: &impl Fn(ArrayView1<f64>, f64) -> Array1<f64>, y0: ArrayView1<f64>, f_k: &mut [Array1<f64>; 13], t0: f64, h: f64) {
    for k in 0..13 {
        let mut y_k = y0.to_owned();
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

fn calculate_error(y0: ArrayView1<f64>, y_new: &mut Array1<f64>, f_k: &[Array1<f64>; 13], h: f64, threshold: f64) -> f64 {
    let mut f_e = E[0] * &f_k[0];
    for i in 5..13 {
        f_e += &(E[i] * &f_k[i]);
    }

    *y_new = &y0 + &((B[0][0]*&f_k[0] + B[0][5]*&f_k[5] + B[0][6]*&f_k[6] + B[0][7]*&f_k[7] + B[0][8]*&f_k[8] + B[0][9]*&f_k[9] + B[0][10]*&f_k[10] + B[0][11]*&f_k[11]) * h);

    let error = Zip::from(&f_e)
        .and(&*y_new)
        .and(&y0)
        .map_collect(|&e, &a, &b| e.abs() / threshold.max(a.abs().max(b.abs())))
        .fold(0., |acc: f64, x| acc.max(*x)) * h;

    error
}

fn update_step(y_i: &mut Array1<f64>, t_i: &mut f64, f_k: &[Array1<f64>; 13], h: f64) {
    *y_i += &((B[0][0]*&f_k[0] + B[0][5]*&f_k[5] + B[0][6]*&f_k[6] + B[0][7]*&f_k[7] + B[0][8]*&f_k[8] + B[0][9]*&f_k[9] + B[0][10]*&f_k[10] + B[0][11]*&f_k[11]) * h);
    *t_i += h;
}

pub fn ode87(fcn: impl Fn(ArrayView1<f64>, f64) -> Array1<f64>, y0: ArrayView1<f64>, tspan: [f64; 2], rtol: f64, atol: f64, h0: Option<f64>) -> Array1<f64> {
    let threshold = atol / rtol;
    let mut remaining = tspan[1] - tspan[0];

    // Calculate initial timestep if no one is provided
    let mut h = match h0 {
        Some(step) => step,
        None => {
            let mut step = (tspan[1] - tspan[0]) / 50.;
            let f0 = fcn(y0.view(), tspan[0]);
            let rh = Zip::from(&f0)
                .and(&y0)
                .map_collect(|&a, &b| a.abs() / threshold.max(b.abs()))
                .fold(0., |res: f64, x| res.max(*x)) / ((7./8.) * rtol.powf(0.125));
            
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
    let mut f_k: [Array1<f64>; 13] = Default::default();

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
            h *= ((7./8.) * (rtol / error).powf(0.125)).max(0.1);
        } else {
            y_i.assign(&y_new);
            t_i += h;
            remaining -= h;

            let tmp = (8./7.) * (error / rtol).powf(0.125);
            h /= tmp.max(0.125);
        }
    }

    y_i
}