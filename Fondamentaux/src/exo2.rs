pub enum LogExp<'a> {
    T,
    F,
    X(usize),
    Non(&'a LogExp<'a>),
    Et(&'a LogExp<'a>, &'a LogExp<'a>),
    Ou(&'a LogExp<'a>, &'a LogExp<'a>),
    Oux(&'a LogExp<'a>, &'a LogExp<'a>)
}

pub fn eval(exp : &LogExp, vals : &Vec<bool>) -> bool {
    match exp {
        LogExp::T => true,
        LogExp::F => false,
        LogExp::X(i) => vals[*i],
        LogExp::Non(e) => !eval(e, &vals),
        LogExp::Et(e1, e2) => eval(e1, &vals) && eval(e2, &vals),
        LogExp::Ou(e1, e2) => eval(e1, &vals) || eval(e2, &vals),
        LogExp::Oux(e1, e2) => eval(e1, &vals) != eval(e2, &vals)
    }
}

pub fn get_max_index(exp : &LogExp) -> usize {
    match exp {
        LogExp::T => 0 as usize,
        LogExp::F => 0 as usize,
        LogExp::X(i) => *i,
        LogExp::Non(e) => get_max_index(e),
        LogExp::Et(e1, e2) => std::cmp::max(get_max_index(e1), get_max_index(e2)),
        LogExp::Ou(e1, e2) => std::cmp::max(get_max_index(e1), get_max_index(e2)),
        LogExp::Oux(e1, e2) => std::cmp::max(get_max_index(e1), get_max_index(e2))
    }
}

pub fn incr(tab : Vec<bool>) -> Vec<bool> {
    let mut carry = true;
    let mut res = vec![false; tab.len()];
    for i in 0..tab.len() {
        res[i] = tab[i] != carry;
        carry = carry && tab[i];
    }
    res
}

pub fn sat(exp : &LogExp) -> Option<Vec<bool>> {
    let mut vals = vec![false; get_max_index(&exp) + 1];
    let cond_arret = vec![true; get_max_index(&exp) + 1];
    while !eval(&exp, &vals) {
        if vals == cond_arret {
            return None
        }
        vals = incr(vals);
    }
    Some(vals)
}
