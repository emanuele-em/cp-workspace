//{"name":"B. Getting Points","group":"Codeforces - Educational Codeforces Round 159 (Rated for Div. 2)","url":"https://codeforces.com/contest/1902/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 5 5 2\n14 3000000000 1000000000 500000000\n100 20 1 10\n8 120 10 20\n42 280 13 37\n","output":"0\n12\n99\n0\n37\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGettingPoints"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size() as isize;
    let mut min = input.read_size() as isize;
    let l = input.read_size() as isize;
    let t = input.read_size() as isize;
    let remaining_days = n % 7 as isize;
    let remaining_task = if remaining_days > 0 {1} else {0} as isize;
    let mut tasks = n / 7 + remaining_task as isize;
    let double_weeks = tasks / 2 as isize;
    let mut worked = 0 as isize;

    if min - double_weeks*(2*t + l) <= 0{
        out.print_line(n - min/(2*t+l) - if min%(2*t+l)>0 {1} else {0});
        return;
    }

    worked += double_weeks;
    tasks -= 2*double_weeks;
    min -= double_weeks*(2*t + l);
    // for _ in 0..double_weeks{
    //     min -= 2*t + l;
    //     tasks -= 2;
    //     worked += 1;
    //     if min <= 0{
    //         out.print_line(n-worked);
    //         return;
    //     }
    // }

    while tasks > 0{
        min -= t + l;
        tasks-=1;
        worked += 1;
        if min <= 0{
            out.print_line(n-worked);
            return;
        }
    }

    out.print_line(n as isize - worked as isize - min/l - if min%l>0 {1} else {0});
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
