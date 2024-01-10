//{"name":"C - Swappable","group":"AtCoder - AtCoder Beginner Contest 206（Sponsored by Panasonic）","url":"https://atcoder.jp/contests/abc206/tasks/abc206_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 7 1\n","output":"2\n"},{"input":"10\n1 10 100 1000 10000 100000 1000000 10000000 100000000 1000000000\n","output":"45\n"},{"input":"20\n7 8 1 1 4 9 9 6 8 2 4 1 1 9 5 5 5 3 6 4\n","output":"173\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSwappable"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_vec::<usize>(n);
    let mut map = std::collections::HashMap::new();
    for &num in &a {
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut res = n*(n-1)/2;
    for freq in map.values(){
        res -= freq*(freq-1)/2;
    }
    out.print_line(res as usize);
    
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
