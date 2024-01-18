//{"name":"B. Summation Game","group":"Codeforces - Codeforces Round 919 (Div. 2)","url":"https://codeforces.com/contest/1920/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 1 1\n1\n4 1 1\n3 1 2 4\n6 6 3\n1 4 3 2 5 6\n6 6 1\n3 7 3 3 32 15\n8 5 3\n5 5 3 3 3 2 9 9\n10 6 4\n1 8 2 9 3 3 4 5 3 200\n2 2 1\n4 3\n2 1 2\n1 3\n","output":"0\n2\n0\n3\n-5\n-9\n0\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSummationGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let alice = input.read_size();
    let bob = input.read_size();
    let mut a = input.read_vec::<isize>(n as usize);
    a.sort_unstable();
    a.reverse();

    let mut full_sum: isize = 0;
    let mut max: isize =isize::MIN;
    for removed in 0..alice+1{
        if removed == 0{
            full_sum = a.iter().sum::<isize>() - a[..bob].iter().sum::<isize>()*2;
        } else {
            let to_add = a[removed-1];
            if removed+bob <= n {
                full_sum = full_sum - a[removed+bob-1]*2 + to_add;
            } else {
                full_sum = full_sum + to_add;
            }
        }
            max = full_sum.max(max);
    }
    out.print_line(max);
    
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
