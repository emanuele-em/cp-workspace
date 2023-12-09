//{"name":"A. Forked!","group":"Codeforces - Codeforces Round 914 (Div. 2)","url":"https://codeforces.com/contest/1904/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 1\n0 0\n3 3\n1 1\n3 1\n1 3\n4 4\n0 0\n8 0\n4 2\n1 4\n3 4\n","output":"2\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AForked"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read::<i32>();
    let b = input.read::<i32>();
    let king = (input.read::<i32>(), input.read::<i32>());
    let queen = (input.read::<i32>(), input.read::<i32>());
    let mut set = std::collections::HashSet::new();
    let possible_positions = [
        (queen.0+a, queen.1+b),
        (queen.0+a, queen.1-b),
        (queen.0-a, queen.1+b),
        (queen.0-a, queen.1-b),
        (queen.0+b, queen.1+a),
        (queen.0+b, queen.1-a),
        (queen.0-b, queen.1+a),
        (queen.0-b, queen.1-a)
    ];
    for position in possible_positions {
        let next_position = [
            (position.0+a, position.1+b),
            (position.0+a, position.1-b),
            (position.0-a, position.1+b),
            (position.0-a, position.1-b),
            (position.0+b, position.1+a),
            (position.0+b, position.1-a),
            (position.0-b, position.1+a),
            (position.0-b, position.1-a)
                    
        ];
        for next in next_position {
            if next == king {
                set.insert(position);
            }
        }
    }
    
    out.print_line(set.len());
    
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
