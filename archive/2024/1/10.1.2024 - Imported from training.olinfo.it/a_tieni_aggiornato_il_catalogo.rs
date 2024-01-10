//{"name":"A. Tieni aggiornato il catalogo","group":"Codeforces - Imported from training.olinfo.it","url":"https://codeforces.com/gym/420506/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\na 5\nc 7\na 10\na 5\nc 5\n","output":"0\n2\n"},{"input":"11\na 11\na 11\na 10\na 10\nc 13\nc 10\na 10\na 10\nt 11\nc 11\nc 10\n","output":"0\n2\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATieniAggiornatoIlCatalogo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc, catalog: &mut std::collections::HashMap<usize, usize>, tot: &mut usize){
    let query = input.read_char();
    let name = input.read_size();
    if query == 'a'{
        catalog.entry(name).and_modify(|x|*x+=1).or_insert(1);
        *tot+=1;
    } else if query == 't'{
        if let Some(is_present) = catalog.get_mut(&name){
            if *is_present > 0{
                *is_present-=1;
                *tot-=1 
            }
        }

    } else {
        if let Some(is_present) = catalog.get_mut(&name){
            out.print_line(*is_present);
        } else {
            out.print_line(0);
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
    let mut catalog = std::collections::HashMap::<usize, usize>::new();
    let mut tot = 0_usize;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc, &mut catalog, &mut tot),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc, &mut catalog, &mut tot);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc, &mut catalog, &mut tot);
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
