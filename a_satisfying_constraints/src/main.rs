//{"name":"A. Satisfying Constraints","group":"Codeforces - Codeforces Round 919 (Div. 2)","url":"https://codeforces.com/contest/1920/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n1 3\n2 10\n3 1\n3 5\n2\n1 5\n2 4\n10\n3 6\n3 7\n1 2\n1 7\n3 100\n3 44\n2 100\n2 98\n1 3\n3 99\n6\n1 5\n2 10\n1 9\n2 2\n3 2\n3 9\n5\n1 1\n2 2\n3 1\n3 2\n3 3\n6\n1 10000\n2 900000000\n3 500000000\n1 100000000\n3 10000\n3 900000001\n","output":"7\n0\n90\n0\n0\n800000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASatisfyingConstraints"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size() as isize;
    let mut l = 0;
    let mut r = 10_isize.pow(9)+1;
    let mut set = vec![];

    for _ in 0..n{
        let t = input.read_size() as isize;
        let k = input.read_size() as isize;
        if t == 1{
            l = k.max(l);
        } else if t == 2{
            r = k.min(r);
        } else {
            set.push(k);
        }
    }

    let mut excluded = 0;
    for num in set{
        if num >= l && num <= r{
            excluded+=1;
        }
    }


    out.print_line(0.max(r-l+1-excluded));
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
