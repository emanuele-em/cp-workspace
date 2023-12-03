//{"name":"C. Insert and Equalize","group":"Codeforces - Educational Codeforces Round 159 (Rated for Div. 2)","url":"https://codeforces.com/contest/1902/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 2 3\n5\n1 -19 17 -3 -15\n1\n10\n","output":"6\n27\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CInsertAndEqualize"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_vec::<isize>(n);
    a.sort();
    let mut min_val = a[0];
    let mut mcm = 0;
    for &num in a.iter().skip(1) {
        mcm = mcm.max(num - min_val);
        min_val = min_val.min(num);
    }
    dbg!(mcm);
    a.push(a.last().unwrap() + mcm);
    let mut ops = 0;
    for i in 1..a.len() {
        let diff = a[i] - a[i-1] as isize;
        a[i] = a[i-1];
        ops += diff / mcm;
    }

    out.print_line(ops);
    
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
