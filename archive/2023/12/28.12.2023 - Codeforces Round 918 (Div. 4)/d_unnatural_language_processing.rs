//{"name":"D. Unnatural Language Processing","group":"Codeforces - Codeforces Round 918 (Div. 4)","url":"https://codeforces.com/contest/1915/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n8\nbacedbab\n4\nbaba\n13\ndaddecabeddad\n3\ndac\n6\ndacdac\n22\ndababbabababbabbababba\n","output":"ba.ced.bab\nba.ba\ndad.de.ca.bed.dad\ndac\ndac.dac\nda.bab.ba.ba.bab.bab.ba.bab.ba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUnnaturalLanguageProcessing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' => true,
        _ => false,
    }
}

fn is_cvc(slice: &[char]) -> bool {
    slice.len() >= 3 && !is_vowel(slice[0]) && is_vowel(slice[1]) && !is_vowel(slice[2])
}

fn is_cv(slice: &[char]) -> bool {
    slice.len() >= 2 && !is_vowel(slice[0]) && is_vowel(slice[1])
}

fn backtracking(start: usize, s: &[char], res: &mut [bool], out: &mut Output) {
    let current = &s[start..];
    if current.len()<=0 {
        return;
    }

    if !is_cvc(current) && !is_cv(current){
        return;
    }
    
    if current.len() == 3 || current.len() == 2 {
        let mut output = String::new();
        for (i, c) in s.iter().enumerate() {
            if res[i] {
                output.push('.');
            }
            output.push(*c);
        }
        out.print_line(format!("{}", output));
        return;
    }

    if is_cv(&current) {
        res[start+2] = true;
        backtracking(start+2, &s, res, out);
        res[start+2] = false;
    }
    if is_cvc(&current) {
        res[start+3] = true;
        backtracking(start+3, &s, res, out);
        res[start+3] = false;
    }

    return;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let start = input.read_vec::<char>(n);
    let mut res = vec![false; n];
    backtracking(0, &mut start.clone(), &mut res, out);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
