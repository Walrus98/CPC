pub fn brute_force(v: &Vec<i32>, k: usize) -> Vec<i32> {

    // lunghezza vettore
    let n = v.len();

    // se k > n, è inutile controllare il sotto array
    if k > n {
        return Vec::<i32>::new();
    }

    // mi creo un vettore che è grande pari al numero di sotto array che avrò grandi K
    let mut maxs = Vec::with_capacity(n - k + 1);

    // per ogni gruppo
    for i in 0..(n - k + 1) {
        // mi prendo il sotto vettore
        let current_slice = &v[i..i + k];
        // mi prendo il max del sotto vettore
        let max_value = *current_slice.iter().max().unwrap();

        // lo aggiungo al vettore
        maxs.push(max_value);
    }

    // ritorno il vettore
    maxs
}

pub fn brute_force_elegant(v: &Vec<i32>, k: usize) -> Vec<i32> {
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}
