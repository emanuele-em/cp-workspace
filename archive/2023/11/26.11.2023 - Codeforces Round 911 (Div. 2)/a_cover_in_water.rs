//{"name":"A. Cover in Water","group":"Codeforces - Codeforces Round 911 (Div. 2)","url":"https://codeforces.com/contest/1900/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n...\n7\n##....#\n7\n..#.#..\n4\n####\n10\n#...#..#.#\n","output":"2\n2\n5\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACoverInWater"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();
    let mut a = input.read_vec::<char>(n);
    let mut len = 0;
    let mut tot = 0;
    for space in a{
        if space == '#'{
            //perform calculation on len
            if len == 1{
                tot += 1;
            } else if len == 2{
                tot += 2
            } else if len > 2{
                out.print_line(2);
                return
            }
            len = 0
        } else {
            len +=1
        }
    }

    if len == 1{
        tot += 1;
    } else if len == 2{
        tot += 2
    } else if len > 2{
        out.print_line(2);
        return
    }

    out.print_line(tot);

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
