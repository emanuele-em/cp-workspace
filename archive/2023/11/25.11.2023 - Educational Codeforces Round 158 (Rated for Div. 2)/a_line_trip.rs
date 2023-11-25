//{"name":"A. Line Trip","group":"Codeforces - Educational Codeforces Round 158 (Rated for Div. 2)","url":"https://codeforces.com/contest/1901/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 7\n1 2 5\n3 6\n1 2 5\n1 10\n7\n","output":"4\n3\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALineTrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
   let n = input.read_int(); 
   let x = input.read_int(); 
   let a = input.read_vec(n as usize);
   let mut max = a[0] as i32;

   for i in 1..n{
        max = max.max(a[i as usize]-a[i as usize-1]);
   }
   out.print_line(max.max((x-a.last().unwrap())*2))
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
