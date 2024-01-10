//{"name":"B. La camera dei cestini","group":"Codeforces - Imported from training.olinfo.it","url":"https://codeforces.com/gym/420506/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 2\ns 0 1\nc 1 0\n","output":"2\n"},{"input":"5 6 7\ns 0 1\nc 1 0\ns 0 2\ns 1 2\ns 0 2\nc 2 2\nc 2 1\n","output":"4\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLaCameraDeiCestini"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let  n = input.read_size();
    let  _m = input.read_size();
    let  q = input.read_size();

    let mut dict = std::collections::HashMap::<usize, Vec<usize>>::new();
    for i in 0..n {
        dict.entry(0).and_modify(|e| e.push(i)).or_insert_with(|| {
            let mut v = vec![];
            v.push(i);
            v
        });
    }

    for _ in 0..q{
        let query = input.read_char();
        let a = input.read_size();
        let second = input.read_size();
        if query == 's'{
            //from top of a to top of second
            if let Some(vec) = dict.get_mut(&a){
                if let Some(element) = vec.pop(){
                    dict.entry(second).and_modify(|x| x.push(element)).or_insert_with(|| {
                        let mut v = vec![];
                        v.push(element);
                        v
                    });
                }
            }

        } else {
            //print the second_th of a
            if let Some(vec) = dict.get_mut(&a){
                if let Some(to_print) = vec.get(second){
                    out.print_line(to_print);
                } else{
                    out.print_line(-1);
                }
            } else{
                out.print_line(-1);
            }
        }
    }
    
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
        _ => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            println!("multinumber");
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            println!("multieof");
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
