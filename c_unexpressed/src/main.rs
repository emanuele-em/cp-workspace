//{"name":"C - Unexpressed","group":"AtCoder - Caddi Programming Contest 2021(AtCoder Beginner Contest 193)","url":"https://atcoder.jp/contests/abc193/tasks/abc193_c","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n","output":"6\n"},{"input":"100000\n","output":"99634\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUnexpressed"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let mut excluded = std::collections::HashSet::<usize>::new();
    let mut pow = 2_usize;
    let mut base = 2_usize;
    while base.pow(2) <= n{
        while base.pow(pow as u32) <= n{
            excluded.insert(base.pow(pow as u32));
            pow += 1;
        }
        pow = 2;
        base+=1;
    }
    out.print_line(n-excluded.len());
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
