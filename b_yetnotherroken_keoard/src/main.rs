//{"name":"B. YetnotherrokenKeoard","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"12\nARaBbbitBaby\nYetAnotherBrokenKeyboard\nBubble\nImprobable\nabbreviable\nBbBB\nBusyasaBeeinaBedofBloomingBlossoms\nCoDEBARbIES\ncodeforces\nbobebobbes\nb\nTheBBlackbboard\n","output":"ity\nYetnotherrokenKeoard\nle\nImprle\nrevile\n\nusyasaeeinaedofloominglossoms\nCDARIES\ncodeforces\nes\n\nhelaoard\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYetnotherrokenKeoard"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = input.read_line();
    let mut res = vec![];
    let mut last_idx_uppercase = vec![];
    let mut last_idx_lowercase = vec![];
    for c in s.chars() {
        if c == 'B' {
            if let Some(last) = last_idx_uppercase.pop() {
                res[last]='*';
            }
        }else if c == 'b' {
            if let Some(last) = last_idx_lowercase.pop() {
                res[last]='*';
            }
        }else {
            if c.is_uppercase() {
                res.push(c);
                last_idx_uppercase.push(res.len()-1);
            } else {
                res.push(c);
                last_idx_lowercase.push(res.len()-1);

            }
        }
    }

    for c in res {
        if c != '*' {
            out.print(c);
        }
    }
    out.print("\n");
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
