use partition_generator::Partgen;

fn main() {
    let partition_style = Partgen::readjson();
    println!("{:#?}", Partgen::readjson());

    Partgen::do_dangerous_task_on("/dev/sdb".to_string(), partition_style);

}