//{"name":"B. Laura and Operations","group":"Codeforces - Codeforces Round 911 (Div. 2)","url":"https://codeforces.com/contest/1900/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1 1\n2 3 2\n82 47 59\n","output":"1 1 1\n0 1 0\n1 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLauraAndOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a = input.read_int();
    let mut b = input.read_int();
    let mut c = input.read_int();
    let res_a = if b == c || ( (b+c)%2==0)  { 1 } else { 0 };
    let res_b = if a == c || ( (a+c)%2==0)  { 1 } else { 0 };
    let res_c = if a == b || ( (b+a)%2==0) { 1 } else { 0 };
    out.print_line(format!("{} {} {}", res_a, res_b, res_c));

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
